package main

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"os/exec"
	"sort"
	"strings"
	"sync"
	"sync/atomic"
	"time"

	"golang.org/x/oauth2"
	"golang.org/x/oauth2/google"
	"google.golang.org/api/gmail/v1"
	"google.golang.org/api/option"
)

func build_query_command(query_list []string) string {
	query_command := "{" + strings.Join(query_list, " ") + "}"
	return query_command
}

func confirm_removal(emails_id []string) string {
	emails_id_len := len(emails_id)
	fmt.Printf("Do you want to remove %d elements? (yes/No): ", emails_id_len)
	var user_response string
	fmt.Scanln(&user_response)

	return user_response
}

func open_file_in_neovim(tmp_file *os.File) []byte {
	// cmd := exec.Command("nvim", "-u", "NONE", "-i", "NONE", tmp_file.Name())
	cmd := exec.Command("nvim", tmp_file.Name())
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		log.Fatalf("Unable to open Neovim: %v", err)
	}

	modified_content, err := os.ReadFile(tmp_file.Name())
	if err != nil {
		log.Fatalf("Unable to read temp file: %v", err)
	}

	return modified_content
}

func edit_emails_in_nvim(email_lines []string, query_command string, flag_query_command bool) []string {
	// Create a temporary file
	tmp_file, err := os.CreateTemp("", "gmail_emails_*.txt")
	if err != nil {
		log.Fatalf("Unable to create temporary file: %v", err)
	}
	defer os.Remove(tmp_file.Name()) // Clean up after Neovim closes

	// Write instructions header
	header := []string{
		"# Actions (prefix each line):",
		"# A (or default) : Archive (Remove from Inbox)",
		"# R, D           : Trash (Delete)",
		"# I              : Ignore (Skip/Keep in Inbox)",
		"# -----------------------------------------------------------",
		"",
	}
	for _, h := range header {
		if _, err := tmp_file.WriteString(h + "\n"); err != nil {
			log.Fatalf("Unable to write to temporary file: %v", err)
		}
	}

	// Write query_command on the file
	// if flag_query_command {
	// 	if _, err := tmp_file.WriteString("query_command: " + query_command + "\n\n"); err != nil {
	// 		log.Fatalf("Unable to write to temporary file: %v", err)
	// 	}
	// }

	// Write email list to the file
	for _, line := range email_lines {
		if _, err := tmp_file.WriteString(line + "\n"); err != nil {
			log.Fatalf("Unable to write to temporary file: %v", err)
		}
	}
	if err := tmp_file.Close(); err != nil {
		log.Fatalf("Unable to close temporary file: %v", err)
	}

	// Open Neovim with the file
	modified_content := open_file_in_neovim(tmp_file)

	// Read the modified content

	// Split content into lines and return
	// Using strings.Split to convert the string into a slice of lines
	return strings.Split(strings.TrimSuffix(string(modified_content), "\n"), "\n")
}

func edit_querylist_file(queries []string) error {
	// open file
	file, err := os.OpenFile("query_list.txt", os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0644)
	if err != nil {
		return fmt.Errorf("unable to open file: %v", err)
	}
	defer file.Close()

	// write items in file
	for _, item := range queries {
		_, err := file.WriteString(fmt.Sprintf("%s\n", item))
		if err != nil {
			return fmt.Errorf("unable to write to file: %v", err)
		}
	}

	return nil
}

func edit_querylist_in_nvim(command_list []string) []string {

	// read query list file
	queries := read_query_file()

	// Create a temporary file
	tmp_file, err := os.CreateTemp("", "query_lists*.txt")
	if err != nil {
		log.Fatalf("Unable to create temporary file: %v", err)
	}
	defer os.Remove(tmp_file.Name()) // Clean up after Neovim closes

	// Write command_list on the file
	if _, err := tmp_file.WriteString("======================\n"); err != nil {
		log.Fatalf("Unable to write to temporary file: %v", err)
	}
	for _, command := range command_list {
		if _, err := tmp_file.WriteString(command + "\n"); err != nil {
			log.Fatalf("Unable to write to temporary file: %v", err)
		}
	}
	if _, err := tmp_file.WriteString("======================\n"); err != nil {
		log.Fatalf("Unable to write to temporary file: %v", err)
	}

	// Write queries to the file
	for _, qi := range queries {
		if _, err := tmp_file.WriteString(qi + "\n"); err != nil {
			log.Fatalf("Unable to write to temporary file: %v", err)
		}
	}

	if err := tmp_file.Close(); err != nil {
		log.Fatalf("Unable to close temporary file: %v", err)
	}

	// Open Neovim with the file
	modified_content := open_file_in_neovim(tmp_file)

	// Split content into lines and return
	content_list := strings.Split(strings.TrimSuffix(string(modified_content), "\n"), "\n")

	// get unique set
	unique_items := make(map[string]bool)

	for _, item := range content_list {
		// ignore separators
		if strings.Contains(item, "====") {
			continue
		}
		// add element to map
		unique_items[item] = true
	}

	filtered_list := make([]string, 0, len(unique_items))

	for item := range unique_items {
		filtered_list = append(filtered_list, item)
	}

	sort.Strings(filtered_list)

	return filtered_list
}

type EmailMsg struct {
	Id      string
	From    string
	Subject string
	Date    string
}

func fetch_messages(srv *gmail.Service, query string) []EmailMsg {
	user := "me"
	// Increase MaxResults to fetch more emails from INBOX
	const maxResults = 400

	// If query is just label:INBOX, we might want to iterate pages?
	// For now, let's just use MaxResults.
	r, err := srv.Users.Messages.List(user).
		Q(query).
		MaxResults(maxResults).Do()

	if err != nil {
		log.Fatalf("Unable to retrieve messages: %v", err)
	}

	var msgs []EmailMsg
	var mu sync.Mutex
	var wg sync.WaitGroup

	// Use a semaphore to limit concurrency
	semaphore := make(chan struct{}, 20)

	// Fetch details in parallel
	for _, m := range r.Messages {
		wg.Add(1)
		go func(m *gmail.Message) {
			defer wg.Done()
			semaphore <- struct{}{}        // Acquire token
			defer func() { <-semaphore }() // Release token

			msgRes, err := srv.Users.Messages.Get(user, m.Id).Format("metadata").Do()
			if err != nil {
				log.Printf("Unable to retrieve message %v: %v", m.Id, err)
				return
			}

			var from, subject, date string
			for _, header := range msgRes.Payload.Headers {
				if header.Name == "From" {
					from = sanitize_header(header.Value)
				}
				if header.Name == "Subject" {
					subject = sanitize_header(header.Value)
				}
				if header.Name == "Date" {
					date = sanitize_header(header.Value)
				}
			}

			mu.Lock()
			msgs = append(msgs, EmailMsg{
				Id:      m.Id,
				From:    from,
				Subject: subject,
				Date:    date,
			})
			mu.Unlock()
		}(m)
	}
	wg.Wait()

	return msgs
}

func sanitize_header(s string) string {
	// Remove newlines and tabs
	s = strings.ReplaceAll(s, "\r", "")
	s = strings.ReplaceAll(s, "\n", " ")
	s = strings.ReplaceAll(s, "\t", " ")
	// Remove pipes as they are our separator
	s = strings.ReplaceAll(s, "|", "-")
	return strings.TrimSpace(s)
}

func matches_query(email EmailMsg, query string) bool {
	query = strings.ToLower(query)
	from := strings.ToLower(email.From)
	subject := strings.ToLower(email.Subject)

	// Clean up query quotes
	clean_query := strings.Trim(query, "\"")

	if strings.HasPrefix(query, "from:") {
		target := strings.TrimPrefix(query, "from:")
		target = strings.Trim(target, "\"")
		return strings.Contains(from, target)
	}

	// Default match: from or subject
	return strings.Contains(from, clean_query) || strings.Contains(subject, clean_query)
}

func filter_messages(emails []EmailMsg, queries []string) []EmailMsg {
	var filtered []EmailMsg
	for _, email := range emails {
		for _, q := range queries {
			if q == "" {
				continue
			}
			if matches_query(email, q) {
				filtered = append(filtered, email)
				break // Match found, add once
			}
		}
	}
	return filtered
}

func messages_to_strings(emails []EmailMsg) []string {
	var lines []string
	for _, email := range emails {
		lines = append(lines, fmt.Sprintf("%s | %s | %s | %s", email.From, email.Subject, email.Date, email.Id))
	}
	sort.Strings(lines)

	// enumerating lines
	// Default action is Archive (A)
	// We prefix with 'A ' so user sees the default action
	for i, line := range lines {
		lines[i] = fmt.Sprintf("A %3d: %s", i+1, line)
	}
	return lines
}

func get_id_from_line(line string) string {
	parts := strings.Split(line, "|")
	if len(parts) > 1 {
		return strings.TrimSpace(parts[len(parts)-1])
	}
	return ""
}

func get_token_from_file(file string) (*oauth2.Token, error) {
	f, err := os.Open(file)
	if err != nil {
		return nil, err
	}
	defer f.Close()
	tok := &oauth2.Token{}
	err = json.NewDecoder(f).Decode(tok)
	return tok, err
}

func get_token_from_web(config *oauth2.Config) *oauth2.Token {
	config.RedirectURL = "urn:ietf:wg:oauth:2.0:oob"
	auth_url := config.AuthCodeURL("state-token", oauth2.AccessTypeOffline)
	fmt.Printf("1. Go to this URL in your browser:\n%v\n", auth_url)
	fmt.Printf("2. After authorizing, copy the code provided.\n")
	fmt.Printf("3. Paste the code here and press Enter: ")

	var auth_code string
	if _, err := fmt.Scan(&auth_code); err != nil {
		log.Fatalf("Unable to read authorization code: %v", err)
	}

	tok, err := config.Exchange(context.TODO(), auth_code)
	if err != nil {
		log.Fatalf("Unable to retrieve token from web: %v", err)
	}
	return tok
}

func init_gmail_srv() (*gmail.Service, error) {
	// read client secret
	b, err := os.ReadFile("client_secret.json")
	if err != nil {
		return nil, err
	}

	// get scope config
	config, err := google.ConfigFromJSON(b, gmail.GmailModifyScope)
	if err != nil {
		return nil, fmt.Errorf("unable to parse client secret file to config: %v", err)
	}

	// read the token
	tok_file := "token.json"
	tok, err := get_token_from_file(tok_file)
	if err != nil {
		tok = get_token_from_web(config)
		save_token(tok_file, tok)
	}

	// get service object
	ctx := context.Background()
	client := config.Client(ctx, tok)
	srv, err := gmail.NewService(ctx, option.WithHTTPClient(client))
	if err != nil {
		return nil, fmt.Errorf("unable to create Gmail service: %v", err)
	}

	return srv, nil
}

func read_actions_from_email_list(content []string) (archive_ids []string, trash_ids []string) {
	// Pattern to find the ID at the end of the line
	// Based on get_id_from_line: uses regex to find ID
	// But get_id_from_line takes the whole line.

	for _, line := range content {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}

		id := get_id_from_line(line)
		if id == "" {
			continue
		}

		// Check for prefixes
		// Convention: 'R' at the start implies Trash (Remove)
		// Everything else (or 'A') implies Archive.

		// We check the first character or word
		parts := strings.Fields(line)
		if len(parts) > 0 {
			action := strings.ToUpper(parts[0])
			switch action {
			case "R", "D":
				trash_ids = append(trash_ids, id)
			case "I":
				// Ignore - do nothing
				continue
			default:
				// Default to archive (A)
				archive_ids = append(archive_ids, id)
			}
		}
	}
	return
}

func read_query_file() []string {
	f, err := os.ReadFile("query_list.txt")
	if err != nil {
		log.Fatalf("Unable to read query_list file")
	}

	queries := strings.Split(strings.ReplaceAll(string(f), "\r", ""), "\n")

	// remove empty values
	result := make([]string, 0)

	for _, qi := range queries {
		if strings.TrimSpace(qi) != "" {
			result = append(result, qi)
		}
	}

	return result
}

func trash_messages(srv *gmail.Service, ids []string) {
	fmt.Printf("Trashing %d messages...\n", len(ids))
	start := time.Now()

	batch_modify_messages(srv, ids, []string{"TRASH"}, []string{"UNREAD", "INBOX"})

	elapsed := time.Since(start).Seconds()
	fmt.Printf("Time taken: %.2f sec\n", elapsed)
}

func archive_messages(srv *gmail.Service, ids []string) {
	fmt.Printf("Archiving %d messages...\n", len(ids))
	start := time.Now()

	// Archive means removing from INBOX. We also remove UNREAD to keep it clean.
	batch_modify_messages(srv, ids, []string{}, []string{"INBOX", "UNREAD"})

	elapsed := time.Since(start).Seconds()
	fmt.Printf("Time taken: %.2f sec\n", elapsed)
}

func batch_modify_messages(srv *gmail.Service, ids []string, addLabels []string, removeLabels []string) {
	modify_request := &gmail.ModifyMessageRequest{
		AddLabelIds:    addLabels,
		RemoveLabelIds: removeLabels,
	}

	total := len(ids)
	if total == 0 {
		return
	}

	var wg sync.WaitGroup
	semaphore := make(chan struct{}, 10) // limit parallel requests
	var processed int32

	for i, id := range ids {
		wg.Add(1)
		semaphore <- struct{}{}
		go func(i int, id string) {
			defer wg.Done()
			defer func() { <-semaphore }()

			_, err := srv.Users.Messages.Modify("me", id, modify_request).Do()
			if err != nil {
				fmt.Printf("\nFailed to modify message %s: %v\n", id, err)
			}

			// Report progress
			current := atomic.AddInt32(&processed, 1)
			if current%5 == 0 || current == int32(total) {
				fmt.Printf("\rProcessing: %d/%d", current, total)
			}
		}(i, id)
	}
	wg.Wait()
	println("\nBatch process complete.")
}

func save_token(path string, token *oauth2.Token) {
	fmt.Printf("Saving credential file to: %s\n", path)
	f, err := os.OpenFile(path, os.O_RDWR|os.O_CREATE|os.O_TRUNC, 0600)
	if err != nil {
		log.Fatalf("Unable to cache oauth token: %v", err)
	}
	defer f.Close()
	json.NewEncoder(f).Encode(token)
}

func select_command(reader *bufio.Reader) string {
	fmt.Printf("select a command: ")
	input, _ := reader.ReadString('\n')
	user_response := strings.TrimSpace(input)

	return user_response
}

func process_messages(srv *gmail.Service, emails []string, query_command string) {
	// edit emails on nvim
	if len(emails) > 0 {
		// edit nvim
		flag_query_command := true
		emails_edited := edit_emails_in_nvim(emails, query_command, flag_query_command)

		archive_ids, trash_ids := read_actions_from_email_list(emails_edited)

		total_ops := len(archive_ids) + len(trash_ids)

		if total_ops == 0 {
			fmt.Println("No emails selected.")
			return
		}

		// confirm with the user
		fmt.Printf("Summary:\n - Archive: %d\n - Trash (Delete): %d\n", len(archive_ids), len(trash_ids))
		fmt.Printf("Proceed? (yes/No): ")
		var user_response string
		fmt.Scanln(&user_response)
		user_response_lower := strings.ToLower(user_response)

		// execute
		if user_response_lower == "y" || user_response_lower == "yes" {
			if len(trash_ids) > 0 {
				trash_messages(srv, trash_ids)
			}
			if len(archive_ids) > 0 {
				archive_messages(srv, archive_ids)
			}
		} else {
			fmt.Println("Operation cancelled.")
		}
	} else {
		fmt.Printf("emails len: %d\n", len(emails))
	}
}

func main() {
	// vars
	reader := bufio.NewReader(os.Stdin)
	commands_list := make([]string, 0)

	// init gmail
	srv, err := init_gmail_srv()
	if err != nil {
		if os.IsNotExist(err) {
			current_dir, _ := os.Getwd()
			fmt.Printf("\nclient_secret.json does not exist\n")
			fmt.Printf("\nHow to fix this:\n\n")
			fmt.Printf("1. goto https://console.cloud.google.com/apis/credentials\n")
			fmt.Printf("2. put the file in `%s` as `client_secret.json`\n\n", current_dir)
			return
		}
		log.Fatalf("Failed to initialize Gmail service: %v", err)
	}

	// read query file
	query_list := read_query_file()
	// instructions
	fmt.Println("Available actions in email list:")
	fmt.Println("  A - Archive (Default)")
	fmt.Println("  R, D - Trash (Delete)")
	fmt.Println("  I - Ignore (Skip/Keep in Inbox)")
	fmt.Println("\nLoop commands:")
	fmt.Println("  e - Edit query list")
	fmt.Println("  . - Refresh INBOX & Filter")
	fmt.Println("  <term> - Search INBOX")

	query_command := "" // Unused for initial local filter but needed for variable scope

	// Initial fetch
	var emails []string

	// Fetch INBOX messages
	msgs := fetch_messages(srv, "label:INBOX")
	fmt.Printf("Fetched %d messages from INBOX search.\n", len(msgs))

	// Filter locally against query list
	spam_msgs := filter_messages(msgs, query_list)
	fmt.Printf("Found %d matches against query_list.\n", len(spam_msgs))

	emails = messages_to_strings(spam_msgs)

	process_messages(srv, emails, query_command)
	for {
		// get command
		command := select_command(reader)
		command_trimmed := strings.TrimSpace(command)

		// Exit conditions
		if command_trimmed == "" || command_trimmed == "/bye" {
			break
		}

		if command_trimmed == "e" {
			// should edit the query_file
			queries_updated := edit_querylist_in_nvim(commands_list)
			edit_querylist_file(queries_updated)
			commands_list = make([]string, 0)

			// Reload query list
			query_list = read_query_file()
			continue
		}

		// fetch and process
		var emails []string

		if command_trimmed == "." {
			// should run all
			// Reload query list to be sure
			query_list = read_query_file()
			query_command = build_query_command(query_list)

			// Refresh logic: Fetch INBOX and filter locally
			msgs := fetch_messages(srv, "label:INBOX")
			fmt.Printf("Fetched %d messages from INBOX.\n", len(msgs))

			spam_msgs := filter_messages(msgs, query_list)
			fmt.Printf("Found %d matches against query_list.\n", len(spam_msgs))

			emails = messages_to_strings(spam_msgs)

		} else {
			// should compute a custom query_command
			// Requirement: search in `inbox` for its value
			// We append label:INBOX to ensure we scope it correctly
			query_command = fmt.Sprintf("label:INBOX %s", command_trimmed)

			commands_list = append(commands_list, command_trimmed)

			// Fetch messages using the custom query
			msgs := fetch_messages(srv, query_command)
			fmt.Printf("Found %d messages matching '%s'.\n", len(msgs), command_trimmed)
			emails = messages_to_strings(msgs)
		}

		process_messages(srv, emails, query_command)
	}

	println("Program terminated")
}
