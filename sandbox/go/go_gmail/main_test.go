package main

import (
	"strings"
	"testing"
)

func TestSanitizeHeader(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"Hello World", "Hello World"},
		{"Hello\nWorld", "Hello World"},
		{"Hello\rWorld", "HelloWorld"}, // ReplaceAll \r with empty string
		{"Hello\tWorld", "Hello World"},
		{"Hello|World", "Hello-World"},
		{"  Hello  ", "Hello"},
	}

	for _, test := range tests {
		result := sanitize_header(test.input)
		if result != test.expected {
			t.Errorf("sanitize_header(%q) = %q; want %q", test.input, result, test.expected)
		}
	}
}

func TestMatchesQuery(t *testing.T) {
	email := EmailMsg{
		From:    "Amazon.com <auto-confirm@amazon.com>",
		Subject: "Your order has shipped",
	}

	tests := []struct {
		query    string
		expected bool
	}{
		{"amazon", true},
		{"from:amazon", true},
		{"order", true},
		{"shipped", true},
		{"google", false},
		{"from:google", false},
		{"\"Amazon.com\"", true},
	}

	for _, test := range tests {
		if matches_query(email, test.query) != test.expected {
			t.Errorf("matches_query(%q) = %v; want %v", test.query, !test.expected, test.expected)
		}
	}
}

func TestParserFlow(t *testing.T) {
	// 1. Setup mock emails
	emails := []EmailMsg{
		{Id: "1", From: "User One", Subject: "Subject One", Date: "2023-01-01"},
		{Id: "2", From: "User Two", Subject: "Subject | With Pipe", Date: "2023-01-02"},
		{Id: "3", From: "User Three", Subject: "Subject\nWith Newline", Date: "2023-01-03"},
	}

	// 2. Filter (mocking filter_messages logic of accepting all for this test)
	// We just proceed to string generation

	// Sanitize first as the main code does in fetch_messages?
	// The main code sanitizes inside fetch_messages.
	// So we should sanitize our mock data to match behavior.
	for i := range emails {
		emails[i].From = sanitize_header(emails[i].From)
		emails[i].Subject = sanitize_header(emails[i].Subject)
	}

	// 3. Convert to lines
	lines := messages_to_strings(emails)

	// Verify lines content
	if len(lines) != 3 {
		t.Errorf("Expected 3 lines, got %d", len(lines))
	}

	for i, line := range lines {
		// Verify default action prefix
		if !strings.HasPrefix(line, "A ") {
			t.Errorf("Line %d should start with 'A ': %s", i, line)
		}
		// Verify sanitation worked (no pipes in subject)
		if strings.Contains(line, "| With Pipe") {
			t.Errorf("Line %d contains unsanitized pipe: %s", i, line)
		}
	}

	// 4. Simulate parser reading back
	archive_ids, trash_ids := read_actions_from_email_list(lines)

	// 5. Verify counts
	total := len(archive_ids) + len(trash_ids)
	if total != 3 {
		t.Errorf("Expected 3 parsed IDs, got %d", total)
	}
	if len(trash_ids) != 0 {
		t.Errorf("Expected 0 trash, got %d", len(trash_ids))
	}
	if len(archive_ids) != 3 {
		t.Errorf("Expected 3 archive, got %d", len(archive_ids))
	}

	// 6. Test Trash parsing
	// Modify one line to simulate user edit
	// Use dynamic expectation: get whatever ID is in the second line.
	expectedTrashId := get_id_from_line(lines[1])
	lines[1] = strings.Replace(lines[1], "A ", "R ", 1)

	archive_ids, trash_ids = read_actions_from_email_list(lines)
	if len(archive_ids) != 2 {
		t.Errorf("Expected 2 archive, got %d", len(archive_ids))
	}
	if len(trash_ids) != 1 {
		t.Errorf("Expected 1 trash, got %d", len(trash_ids))
	}
	if trash_ids[0] != expectedTrashId {
		t.Errorf("Expected Trash ID %s, got %s", expectedTrashId, trash_ids[0])
	}
}

func TestGetIdFromLine(t *testing.T) {
	tests := []struct {
		line     string
		expected string
	}{
		{"A   1: From | Subject | Date | 12345", "12345"},
		{"R   2: From | Subject | Date | abcde", "abcde"},
		{"   3: From | Subject | Date | xyz", "xyz"}, // No prefix
		{"A 100: Name | Subj | Date | id_with_underscore", "id_with_underscore"},
		{"R 999: Pipe | In | Content | 555", "555"}, // Pipe in content should have been sanitized, but if not?
		// if pipe is in content, get_id matches last part.
		{"A   1: F | S | D | 123", "123"},
	}

	for _, test := range tests {
		result := get_id_from_line(test.line)
		if result != test.expected {
			t.Errorf("get_id_from_line(%q) = %q; want %q", test.line, result, test.expected)
		}
	}
}
