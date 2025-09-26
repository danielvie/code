import * as vscode from 'vscode';
// NOTE: You must run 'npm install js-yaml' in your extension's terminal for this import to work.
import * as yaml from 'js-yaml'; 
import * as fs from 'fs';
import * as path from 'path';

// Define the name of the configuration file to read
const YAML_FILE_NAME = 'config.yml';

// ==================================================================================
// 1. Data Model (TreeItem Class)
// ==================================================================================

type NodeType = 'map' | 'array' | 'scalar' | 'root';
type PathSegment = string | number;

class YamlTreeItem extends vscode.TreeItem {
  constructor(
    public readonly path: PathSegment[],  
    public readonly key: PathSegment,     
    public readonly value: any,           
    public readonly nodeType: NodeType,  
    label: string,                       
    collapsibleState: vscode.TreeItemCollapsibleState, 
  ) {
    super(label, collapsibleState);
    
    switch (nodeType) {
      case 'map':
        this.iconPath = new vscode.ThemeIcon('folder-opened'); 
        break;
      case 'array':
        this.iconPath = new vscode.ThemeIcon('list-ordered'); 
        break;
      case 'scalar':
        this.iconPath = new vscode.ThemeIcon('symbol-property');
        this.description = String(value); 
        break;
      default:
        this.iconPath = new vscode.ThemeIcon('settings-gear');
    }

    this.tooltip = this.getTooltipText();
    this.contextValue = nodeType === 'scalar' ? 'yamlScalarItem' : nodeType; 
  }

  private getTooltipText(): string {
      if (this.nodeType === 'scalar') {
          return `${this.path.join('.')}: ${this.value}`;
      }
      const labelContent = typeof this.label === 'string' 
          ? this.label 
          : (this.label?.label ?? this.key);
      
      return String(labelContent);
  }
}


// ==================================================================================
// 2. YAML Data Provider (TreeDataProvider Class)
// ==================================================================================

class YamlTreeDataProvider implements vscode.TreeDataProvider<YamlTreeItem> {
  private _onDidChangeTreeData: vscode.EventEmitter<YamlTreeItem | undefined | void> = 
      new vscode.EventEmitter<YamlTreeItem | undefined | void>();
  readonly onDidChangeTreeData: vscode.Event<YamlTreeItem | undefined | void> = 
      this._onDidChangeTreeData.event;
  
  private parsedYamlData: any = {};
  private filterTerm: string | undefined = undefined; 

  constructor(private workspaceRoot: string | undefined) {
    if (this.workspaceRoot) {
        this.readAndParseYamlFile();
    }
  }

  public refresh(): void {
    if (this.workspaceRoot) {
        this.readAndParseYamlFile();
    }
    this._onDidChangeTreeData.fire();
  }
  
  public setFilter(term: string | undefined): void {
      this.filterTerm = term ? term.toLowerCase().trim() : undefined;
      this._onDidChangeTreeData.fire(); 
  }

  private readAndParseYamlFile(): void {
    if (!this.workspaceRoot) return;
    const filePath = path.join(this.workspaceRoot, YAML_FILE_NAME);
    
    try {
      const fileContent = fs.readFileSync(filePath, 'utf8');
      this.parsedYamlData = yaml.load(fileContent);
      if (typeof this.parsedYamlData !== 'object' || this.parsedYamlData === null) {
        this.parsedYamlData = { error: 'YAML file is empty or invalid format.' };
      }
    } catch (error) {
      this.parsedYamlData = { 
          error: `Could not read ${YAML_FILE_NAME}`,
          details: (error as Error).message 
      };
      vscode.window.showErrorMessage(`Error reading YAML file: ${ (error as Error).message }`);
    }
  }

  private saveYamlFile(): void {
    if (!this.workspaceRoot) return;
    const filePath = path.join(this.workspaceRoot, YAML_FILE_NAME);
    
    try {
        const yamlContent = yaml.dump(this.parsedYamlData, { indent: 2, lineWidth: -1 });
        fs.writeFileSync(filePath, yamlContent, 'utf8');
        vscode.window.showInformationMessage(`${YAML_FILE_NAME} saved successfully.`);
    } catch (error) {
        vscode.window.showErrorMessage(`Error writing YAML file: ${ (error as Error).message }`);
    }
  }

  public updateValue(node: YamlTreeItem, newValue: string): void {
    let current: any = this.parsedYamlData;
    
    for (let i = 0; i < node.path.length - 1; i++) {
        const segment = node.path[i];
        if (current && typeof current === 'object' && Object.prototype.hasOwnProperty.call(current, segment)) {
            current = current[segment];
        } else {
            vscode.window.showErrorMessage(`Failed to traverse path segment: ${segment}. Cannot update data.`);
            return;
        }
    }

    if (current && typeof current === 'object' && Object.prototype.hasOwnProperty.call(current, node.key)) {
        current[node.key] = newValue;
        this.saveYamlFile();
        this.refresh();
    } else {
        vscode.window.showErrorMessage(`Could not find final key: ${node.key}. Update failed.`);
    }
  }
  
  private passesFilter(value: any, key: PathSegment): boolean {
    if (!this.filterTerm) {
        return true; 
    }
    
    if (String(key).toLowerCase().includes(this.filterTerm)) {
        return true;
    }

    if (typeof value !== 'object' || value === null) {
        if (String(value).toLowerCase().includes(this.filterTerm)) {
            return true;
        }
        return false; 
    }
    
    if (typeof value === 'object' && value !== null) {
        const iterable = Array.isArray(value) ? value.entries() : Object.entries(value);
        for (const [childKey, childValue] of iterable) {
            if (this.passesFilter(childValue, childKey)) {
                return true;
            }
        }
    }

    return false;
  }
  
  getTreeItem(element: YamlTreeItem): vscode.TreeItem { return element; }

  getChildren(element?: YamlTreeItem): Thenable<YamlTreeItem[]> {
    if (!this.workspaceRoot) {
        const message = 'Open a workspace folder to view YAML data.';
        return Promise.resolve([
            new YamlTreeItem(['Info'], 'Info', message, 'root', 'Please Open Folder', vscode.TreeItemCollapsibleState.None)
        ]);
    }
    
    let targetValue: any = this.parsedYamlData;
    let parentPath: PathSegment[] = [];

    if (element) {
        targetValue = element.value;
        parentPath = element.path;
    }

    const children: YamlTreeItem[] = [];

    if (typeof targetValue === 'object' && targetValue !== null) {
      if (Array.isArray(targetValue)) {
        targetValue.forEach((item, index) => {
          const itemKey = index;
          const itemPath = [...parentPath, itemKey];
          children.push(this.createTreeItem(itemPath, itemKey, item));
        });
      } else {
        Object.keys(targetValue).forEach(key => {
          const itemPath = [...parentPath, key];
          children.push(this.createTreeItem(itemPath, key, targetValue[key]));
        });
      }
    }

    if (this.filterTerm && !element) {
        return Promise.resolve(children.filter(child => this.passesFilter(child.value, child.key)));
    }

    return Promise.resolve(children);
  }
  
  private createTreeItem(path: PathSegment[], key: PathSegment, value: any): YamlTreeItem {
    const label = (typeof key === 'number') ? `[${key}]` : key;
    const isExpandable = typeof value === 'object' && value !== null && 
                         (Array.isArray(value) ? value.length > 0 : Object.keys(value).length > 0);
    
    const nodeType: NodeType = Array.isArray(value) 
        ? 'array' 
        : (typeof value === 'object' && value !== null ? 'map' : 'scalar');
    
    const collapsibleState = isExpandable
        ? vscode.TreeItemCollapsibleState.Collapsed
        : vscode.TreeItemCollapsibleState.None;

    return new YamlTreeItem(path, key, value, nodeType, label.toString(), collapsibleState);
  }
}

// ==================================================================================
// 3. Webview View Provider (New Block)
// ==================================================================================

class FormViewProvider implements vscode.WebviewViewProvider {
    public static readonly viewType = 'my-form-view'; 
    private _view?: vscode.WebviewView;

    constructor(private readonly _extensionUri: vscode.Uri) {}

    public resolveWebviewView(
        webviewView: vscode.WebviewView,
        context: vscode.WebviewViewResolveContext,
        token: vscode.CancellationToken,
    ): void {
        console.log('FormViewProvider.resolveWebviewView called');
        
        this._view = webviewView;

        // Configure webview options
        webviewView.webview.options = {
            enableScripts: true,
            localResourceRoots: [this._extensionUri]
        };

        // Set the HTML content
        webviewView.webview.html = this._getHtmlContent(webviewView.webview);

        // Handle messages from webview
        webviewView.webview.onDidReceiveMessage(
            message => {
                console.log('Received message from webview:', message);
                switch (message.command) {
                    case 'submit':
                        vscode.window.showInformationMessage(
                            `Form submitted: ${message.appName}, ${message.port}`
                        );
                        return;
                }
            },
            undefined,
            []
        );

        console.log('FormViewProvider.resolveWebviewView completed');
    }

    private _getHtmlContent(webview: vscode.Webview): string {
        const nonce = this._getNonce();
        
        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}';">
    <title>Settings Form</title>
    <style>
        body {
            font-family: var(--vscode-font-family);
            color: var(--vscode-foreground);
            background-color: var(--vscode-editor-background);
            padding: 20px;
        }
        .form-group {
            margin-bottom: 15px;
        }
        label {
            display: block;
            margin-bottom: 5px;
            font-weight: bold;
        }
        input[type="text"], input[type="number"] {
            width: 100%;
            padding: 8px;
            box-sizing: border-box;
            border: 1px solid var(--vscode-input-border);
            background-color: var(--vscode-input-background);
            color: var(--vscode-input-foreground);
            border-radius: 3px;
        }
        button {
            background-color: var(--vscode-button-background);
            color: var(--vscode-button-foreground);
            border: none;
            padding: 10px 15px;
            border-radius: 3px;
            cursor: pointer;
            font-size: 13px;
        }
        button:hover {
            background-color: var(--vscode-button-hoverBackground);
        }
    </style>
</head>
<body>
    <h3>Application Settings</h3>
    <form>
        <div class="form-group">
            <label for="appName">Application Name:</label>
            <input type="text" id="appName" value="MyWebApp" />
        </div>
        <div class="form-group">
            <label for="port">Server Port:</label>
            <input type="number" id="port" value="8080" />
        </div>
        <button type="button" id="submitBtn">Save Settings</button>
    </form>

    <script nonce="${nonce}">
        const vscode = acquireVsCodeApi();
        
        document.getElementById('submitBtn').addEventListener('click', function() {
            const appName = document.getElementById('appName').value;
            const port = document.getElementById('port').value;
            
            vscode.postMessage({
                command: 'submit',
                appName: appName,
                port: port
            });
        });
    </script>
</body>
</html>`;
    }

    private _getNonce(): string {
        let text = '';
        const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
        for (let i = 0; i < 32; i++) {
            text += possible.charAt(Math.floor(Math.random() * possible.length));
        }
        return text;
    }
}

// ==================================================================================
// 4. Extension Activation
// ==================================================================================

export function activate(context: vscode.ExtensionContext) {
  console.log('YAML Config Tree Extension is now active!');

  const workspaceRoot = (vscode.workspace.workspaceFolders && (vscode.workspace.workspaceFolders.length > 0))
    ? vscode.workspace.workspaceFolders[0].uri.fsPath
    : undefined;

  // --- YAML Tree View Registration ---
  const treeDataProvider = new YamlTreeDataProvider(workspaceRoot);
  const treeView = vscode.window.createTreeView('my-extension-sidebar-view', {
    treeDataProvider: treeDataProvider,
    showCollapseAll: true 
  });
  console.log('SUCCESS: Tree view registered for my-extension-sidebar-view');

  // --- Webview Form View Registration ---
  console.log('Attempting to register webview view provider...');
  try {
    const formProvider = new FormViewProvider(context.extensionUri);
    console.log('FormViewProvider instance created');
    
    const formProviderDisposable = vscode.window.registerWebviewViewProvider(
        'my-form-view', // Use literal string instead of static property for debugging
        formProvider,
        { 
            webviewOptions: { 
                retainContextWhenHidden: true 
            } 
        }
    );
    
    console.log('WebviewViewProvider registered successfully for view: my-form-view');
    context.subscriptions.push(formProviderDisposable);
    
  } catch (error) {
    console.error('CRITICAL ERROR: Failed to register FormViewProvider:', error);
    vscode.window.showErrorMessage(`Failed to register webview provider: ${error}`);
  }

  // -------------------------------------------------------------
  // COMMAND REGISTRATION
  // -------------------------------------------------------------

  // 1. Refresh Command
  let disposableRefresh = vscode.commands.registerCommand('my-extension-sidebar-view.refresh', () => {
      treeDataProvider.refresh();
      vscode.window.showInformationMessage(`Refreshed ${YAML_FILE_NAME} data.`);
  });

  // 2. Edit Value Command
  let disposableEdit = vscode.commands.registerCommand('my-extension-sidebar-view.editValue', async (node: YamlTreeItem) => {
    
    if (node.nodeType !== 'scalar') {
        vscode.window.showWarningMessage('Only single key-value entries (scalars) can be edited.');
        return;
    }
    
    const newValue = await vscode.window.showInputBox({
        title: `Edit: ${node.path.join('.')}`,
        value: String(node.value),
        prompt: 'Enter the new value for this setting.'
    });

    if (newValue !== undefined) {
        treeDataProvider.updateValue(node, newValue);
    }
  });
  
  // 3. Filter Command 
  let disposableFilter = vscode.commands.registerCommand('my-extension-sidebar-view.setFilter', async () => {
      const currentFilter = treeDataProvider['filterTerm'] || '';
      
      const newFilter = await vscode.window.showInputBox({
          title: 'Filter YAML Tree (Recursive Search)',
          value: currentFilter,
          prompt: 'Enter text to filter by key or value. Leave blank to clear.'
      });
      
      if (newFilter !== undefined) {
          treeDataProvider.setFilter(newFilter);
          vscode.window.showInformationMessage(newFilter ? `Filter applied: "${newFilter}"` : 'Filter cleared.');
      }
  });

  // 4. Form Submission Command
  let disposableFormSubmit = vscode.commands.registerCommand('my-extension-sidebar-view.submitForm', () => {
      // This command is reserved if you want to trigger form actions from external sources
  });

  // Collect all disposables
  context.subscriptions.push(
      disposableRefresh, 
      disposableEdit, 
      disposableFilter, 
      disposableFormSubmit, 
      treeView
      // formProviderDisposable is already added inside the try-catch block above
  );
}

export function deactivate() {}
