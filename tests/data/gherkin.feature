Feature: Counting lines of code in Gherkin files

  This is some freeform text information, but it is not a comment.
  # ...but this is a comment.

  Scenario: File is counted
   Given a file with extension ".feature"
    When loc processes it
    Then it should correctly count the number of lines
     And it should correctly count the number of comments
     # And it should solve world hunger
     And we should not not get ahead of ourselves
