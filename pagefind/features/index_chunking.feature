@skip
Feature: Index Chunking

    Background:
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |

    Scenario: Browser only loads chunks needed to search for the target word
    Scenario: Chunk size is configurable
