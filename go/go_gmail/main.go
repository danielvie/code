package main

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"os/exec"
	"regexp"
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

// Retrieve a token, saves the token, then returns the generated client.
func initialize_gmail_service() (*gmail.Service, error) {
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
	tok, err := token_from_file(tok_file)
	if err != nil {
		tok = get_token_from_web(config)
		save_token(tok_file, tok)
	}

	// get service object
	ctx := context.Background()
	client := config.Client(ctx, tok)
	srv, err := gmail.NewService(ctx, option.WithHTTPClient(client))
	if err != nil {
		return nil, fmt.Errorf("Unable to create Gmail service: %v", err)
	}

	return srv, nil
}

// Request a token from the web, then returns the retrieved token.
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

// Retrieve a token from a local file.
func token_from_file(file string) (*oauth2.Token, error) {
	f, err := os.Open(file)
	if err != nil {
		return nil, err
	}
	defer f.Close()
	tok := &oauth2.Token{}
	err = json.NewDecoder(f).Decode(tok)
	return tok, err
}

// Save a token to a file path.
func save_token(path string, token *oauth2.Token) {
	fmt.Printf("Saving credential file to: %s\n", path)
	f, err := os.OpenFile(path, os.O_RDWR|os.O_CREATE|os.O_TRUNC, 0600)
	if err != nil {
		log.Fatalf("Unable to cache oauth token: %v", err)
	}
	defer f.Close()
	json.NewEncoder(f).Encode(token)
}

// List emails and return them as a string slice
func fetch_emails(srv *gmail.Service, query_command string) []string {
	user := "me"
	r, err := srv.Users.Messages.List(user).
		LabelIds("INBOX").
		Q(query_command).
		// Q("from:\"liquidz\" OR from:\"nubank\"").
		MaxResults(100).Do()

	// r, err := srv.Users.Messages.List(user).Do()
	if err != nil {
		log.Fatalf("Unable to retrieve messages: %v", err)
	}

	var email_lines []string

	for _, m := range r.Messages {
		msg, err := srv.Users.Messages.Get(user, m.Id).Do()
		if err != nil {
			log.Printf("Unable to retrieve message %v: %v", m.Id, err)
			continue
		}

		var from, subject, date string
		for _, header := range msg.Payload.Headers {
			if header.Name == "From" {
				from = header.Value
			}
			if header.Name == "Subject" {
				subject = header.Value
			}
			if header.Name == "Date" {
				date = header.Value
			}
		}

		email_lines = append(email_lines, fmt.Sprintf("%s | %s | %s | %s", from, subject, date, m.Id))
	}
	// sorting emails
	sort.Strings(email_lines)

	// enumerating lines
	for i, email := range email_lines {
		email_lines[i] = fmt.Sprintf("%3d: %s", i+1, email)
	}

	return email_lines
}

// Write email list to a file and open it in Neovim
func edit_emails_in_nvim(email_lines []string, query_command string, flag_query_command bool) []string {
	// Create a temporary file
	tmp_file, err := os.CreateTemp("", "gmail_emails_*.txt")
	if err != nil {
		log.Fatalf("Unable to create temporary file: %v", err)
	}
	defer os.Remove(tmp_file.Name()) // Clean up after Neovim closes

	// Write query_command on the file
	if flag_query_command {
		if _, err := tmp_file.WriteString("query_command: " + query_command + "\n\n"); err != nil {
			log.Fatalf("Unable to write to temporary file: %v", err)
		}
	}

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
	cmd := exec.Command("nvim", tmp_file.Name())
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		log.Fatalf("Unable to open Neovim: %v", err)
	}

	// Read the modified content
	modified_content, err := os.ReadFile(tmp_file.Name())
	if err != nil {
		log.Fatalf("Unable to read temp file: %v", err)
	}

	// Split content into lines and return
	// Using strings.Split to convert the string into a slice of lines
	return strings.Split(strings.TrimSuffix(string(modified_content), "\n"), "\n")
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
	cmd := exec.Command("nvim", tmp_file.Name())
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		log.Fatalf("Unable to open Neovim: %v", err)
	}

	// Read the modified content
	modified_content, err := os.ReadFile(tmp_file.Name())
	if err != nil {
		log.Fatalf("Unable to read temp file: %v", err)
	}

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

func find_id(line string) string {
	// regex the pattern
	pattern := `^\s+\d+`
	re := regexp.MustCompile(pattern)

	// regex the pattern to find the message id
	re_id := regexp.MustCompile(`.*\|`)

	// find matches
	var msg_id string
	if matches := re.FindStringSubmatch(line); len(matches) > 0 {
		msg_id = strings.TrimSpace(re_id.ReplaceAllString(line, ""))
	}

	return msg_id
}

func read_query_file() []string {
	f, err := os.ReadFile("query_list.txt")
	if err != nil {
		log.Fatalf("Unable to read query_list file")
	}

	queries := strings.Split(strings.ReplaceAll(string(f), "\r", ""), "\n")

	return queries
}

func remove_messages(srv *gmail.Service, ids []string) {
	start := time.Now()

	modify_request := &gmail.ModifyMessageRequest{
		RemoveLabelIds: []string{"UNREAD", "INBOX"},
	}

	total := len(ids)
	var wg sync.WaitGroup
	semaphore := make(chan struct{}, 5) // limit to 5 parallel requests
	var processed int32

	for i, id := range ids {
		wg.Add(1)
		semaphore <- struct{}{} // acquire a slot in the semaphore
		go func(i int, id string) {
			defer wg.Done()
			defer func() { <-semaphore }() // release the slot

			_, err := srv.Users.Messages.Modify("me", id, modify_request).Do()
			if err != nil {
				fmt.Printf("\nFailed to modify message %s: %v\n", id, err)
			}

			// increment the processed counter
			current := atomic.AddInt32(&processed, 1)
			fmt.Printf("\rProcessing: %d/%d", current, total)

		}(i, id)
	}

	wg.Wait()

	println("\n\nAll messages processed!")

	elapsed := time.Since(start).Seconds()
	fmt.Printf("Time taken: %.2f sec\n", elapsed)
}

func build_query_command(query_list []string) string {
	for i, qi := range query_list {
		query_list[i] = fmt.Sprintf("from:%s", strings.ReplaceAll(qi, "\"", "\\\""))
	}

	query_command := strings.Join(query_list, " OR ")
	query_command = fmt.Sprintf("(%s)", query_command)

	return query_command
}

func read_ids_from_email_list(content []string) []string {
	ids_to_remove := make([]string, 0, len(content))

	for _, c := range content {
		id := find_id(c)
		if len(id) > 0 {
			ids_to_remove = append(ids_to_remove, id)
		}
	}

	return ids_to_remove
}

func confirm_removal(emails_id []string) string {
	emails_id_len := len(emails_id)
	fmt.Printf("Do you want to remove %d elements? (Yes/No): ", emails_id_len)
	var user_response string
	fmt.Scanln(&user_response)

	return user_response
}

func get_command(reader *bufio.Reader) string {
	fmt.Printf("select a command: ")
	input, _ := reader.ReadString('\n')
	user_response := strings.TrimSpace(input)

	return user_response
}

func main() {
	// vars
	reader := bufio.NewReader(os.Stdin)
	break_out := false
	commands_list := make([]string, 0)

	// init gmail
	srv, err := initialize_gmail_service()
	if err != nil {
		if os.IsNotExist(err) {
			current_dir, _ := os.Getwd()
			fmt.Printf("\nclient_secret.json does not exist\n")
			fmt.Printf("\nHow to fix this:\n\n")
			fmt.Printf("1. goto https://console.cloud.google.com/apis/credentials\n")
			fmt.Printf("2. put the file in `%s` as `credential.json`\n\n", current_dir)
			return
		}
		log.Fatalf("Failed to initialize Gmail service: %v", err)
	}

	// read query file
	query_list := read_query_file()
	query_command := build_query_command(query_list)

	for {
		// process query_command
		emails := fetch_emails(srv, query_command)

		// edit emails on nvim
		if len(emails) > 0 {
			// edit nvim
			flag_query_command := true
			emails_edited := edit_emails_in_nvim(emails, query_command, flag_query_command)
			emails_id := read_ids_from_email_list(emails_edited)

			// confirm with the user to remove ids
			user_response := confirm_removal(emails_id)
			user_response_lower := strings.ToLower(user_response)

			// remove emails
			if user_response_lower == "y" || user_response_lower == "yes" {
				fmt.Println("Proceeding with removal...")
				remove_messages(srv, emails_id)
			}
		} else {
			fmt.Printf("emails len: %d\n", len(emails))
		}

		// get command
		command := get_command(reader)

		switch strings.ToLower(command) {
		case ".":
			// should run all
			query_list = read_query_file()
			query_command = build_query_command(query_list)
		case "":
			// should break out
			break_out = true
		case "e":
			// should edit the query_file
			queries_updated := edit_querylist_in_nvim(commands_list)
			edit_querylist_file(queries_updated)

		default:
			// should compute a custom query_command
			query_command = build_query_command([]string{command})
			commands_list = append(commands_list, command)
		}

		// prompt for new input
		if break_out {
			break
		}
	}

	println("Program terminated")

}
