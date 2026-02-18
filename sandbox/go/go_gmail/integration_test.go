package main

import (
	"os"
	"testing"
)

// This test fetches REAL data and prints out why emails are being skipped.
// Run with: go test -v -run TestRealIntegration
func TestRealIntegration(t *testing.T) {
	// 1. Check if credentials exist, potential skip
	if _, err := os.Stat("client_secret.json"); os.IsNotExist(err) {
		t.Skip("client_secret.json not found, skipping integration test")
	}

	// 2. Initialize Service
	srv, err := init_gmail_srv()
	if err != nil {
		t.Fatalf("Failed to init gmail service: %v", err)
	}

	// 3. Read Query List
	query_list := read_query_file()
	t.Logf("Loaded %d queries from query_list.txt", len(query_list))

	// 4. Fetch INBOX
	t.Log("Fetching INBOX messages (max 400)...")
	msgs := fetch_messages(srv, "label:INBOX")
	t.Logf("Fetched %d total messages from INBOX", len(msgs))

	// 5. Filter
	matched := filter_messages(msgs, query_list)
	t.Logf("Matched %d messages against query list", len(matched))

	// 6. Analyze Mismatches
	// Create a map of matched IDs for quick lookup
	matchedMap := make(map[string]bool)
	for _, m := range matched {
		matchedMap[m.Id] = true
	}

	skippedCount := 0
	t.Log("\n--- SKIPPED EMAILS (Did not match query_list) ---")
	for _, m := range msgs {
		if !matchedMap[m.Id] {
			skippedCount++
			// Print the first 20 skipped to avoid spamming the log, or all if user wants
			if skippedCount <= 50 {
				t.Logf("[%d] FILTERED OUT:\n  From:    %s\n  Subject: %s\n  Date:    %s\n",
					skippedCount, m.From, m.Subject, m.Date)
			}
		}
	}

	if skippedCount > 0 {
		t.Logf("\nTotal Skipped: %d. Check the logs above to see if these SHOULD have matched.", skippedCount)
	} else {
		t.Log("All emails matched!")
	}
}
