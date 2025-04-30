package main

import (
	"fmt"
)

// Interface with Start and Stop methods
type Playable interface {
	Start()
	Stop()
}

// Movie struct with title and duration
type Movie struct {
	Title    string
	Duration int // in minutes
}

// Implement Start method for Movie
func (m Movie) Start() {
	fmt.Printf("Starting the movie: %s\n", m.Title)
}

// Implement Stop method for Movie
func (m Movie) Stop() {
	fmt.Printf("Stopping the movie: %s\n", m.Title)
}

// Additional method specific to Movie
func (m Movie) ShowCredits() {
	fmt.Println("Showing movie credits...")
}

// Music struct with title and artist
type Music struct {
	Title  string
	Artist string
}

// Implement Start method for Music
func (m Music) Start() {
	fmt.Printf("Playing the song: %s by %s\n", m.Title, m.Artist)
}

// Implement Stop method for Music
func (m Music) Stop() {
	fmt.Printf("Stopping the song: %s\n", m.Title)
}

// Additional method specific to Music
func (m Music) DisplayLyrics() {
	fmt.Println("Displaying lyrics...")
}

// Function that accepts a Playable interface
func PlayMedia(p Playable) {
	p.Start()
	p.Stop()
}

func main() {
	// Create instances
	movie := Movie{Title: "Inception", Duration: 148}
	music := Music{Title: "Bohemian Rhapsody", Artist: "Queen"}

	// Use the interface
	PlayMedia(movie)
	movie.ShowCredits()

	fmt.Println()

	PlayMedia(music)
	music.DisplayLyrics()
}
