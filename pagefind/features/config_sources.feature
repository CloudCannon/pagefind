Feature: Config Sources

    Scenario: Settings can be pulled from TOML configuration files
        Given I have a "public/index.html" file with the body:
            """
            <h1>Hello.</h1>
            """
        Given I have a "pagefind.toml" file with the content:
            """
            site = "public"
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"

    Scenario: Settings can be pulled from YAML configuration files
        Given I have a "public/index.html" file with the body:
            """
            <h1>Hello.</h1>
            """
        Given I have a "pagefind.yml" file with the content:
            """
            site: public
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"

    Scenario: Settings can be pulled from JSON configuration files
        Given I have a "public/index.html" file with the body:
            """
            <h1>Hello.</h1>
            """
        Given I have a "pagefind.json" file with the content:
            """
            {
                "site": "public"
            }
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"

    Scenario: Settings can be pulled from command-line flags
        Given I have a "public/index.html" file with the body:
            """
            <h1>Hello.</h1>
            """
        When I run my program with the flags:
            | --site public |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"

    Scenario: Settings can be pulled from environment variables
        Given I have a "public/index.html" file with the body:
            """
            <h1>Hello.</h1>
            """
        Given I have the environment variables:
            | PAGEFIND_SITE | public |
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/pagefind/pagefind.js"

    Scenario: Settings can be pulled from multiple sources
        Given I have a "public/index.html" file with the body:
            """
            <h1>Hello.</h1>
            """
        Given I have a "pagefind.json" file with the content:
            """
            {
                "site": "public"
            }
            """
        When I run my program with the flags:
            | --output-subdir _out |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_out/pagefind.js"
