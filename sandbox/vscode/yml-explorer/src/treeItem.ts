import * as vscode from 'vscode';

export class SimpleTreeItem extends vscode.TreeItem {
  constructor(
    public readonly label: string, // The text displayed
    public readonly collapsibleState: vscode.TreeItemCollapsibleState,
    public readonly command?: vscode.Command,
  ) {
    super(label, collapsibleState);
    this.tooltip = `${this.label} item`; // Optional tooltip
  }

  // Optionally set a context value for menu commands
  contextValue = 'simpleTreeItem'; 
}