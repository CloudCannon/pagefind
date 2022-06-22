@skip
Feature: Spellcheck

    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |

    Scenario: Spelling correction can be returned for the unique words in the dataset
