package utils

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPathExists(t *testing.T) {
	// Test that the current directory exists
	assert.True(t, PathExists("."), "Current directory should exist")
	
	// Test that a non-existent path doesn't exist
	assert.False(t, PathExists("./non-existent-directory-12345"), "Non-existent directory should not exist")
}

func TestIsPathValid(t *testing.T) {
	// Test that the current directory is valid
	assert.True(t, IsPathValid("."), "Current directory should be valid")
	
	// Test that a non-existent path is not valid
	assert.False(t, IsPathValid("./non-existent-directory-12345"), "Non-existent directory should not be valid")
}