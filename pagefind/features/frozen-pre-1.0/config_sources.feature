# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
# This file represents a backwards-compatible setup as it existed before 1.0  #
# These tests should remain as a permanent regresison check for older sites   #
# It is very unlikely that the tests in this file should be touched           #
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #

Feature: Config Sources

    Scenario: LEGACY Settings can be pulled from TOML configuration files
        Given I have a "public/index.html" file with the body:
            """
            <link rel="pre-1.0-signal" href="_pagefind" >
            <h1>Hello.</h1>
            """
        Given I have a "pagefind.toml" file with the content:
            """
            source = "public"
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see "pre-1.0 compatibility mode" in stderr
        Then I should see the file "public/_pagefind/pagefind.js"

    Scenario: LEGACY Settings can be pulled from YAML configuration files
        Given I have a "public/index.html" file with the body:
            """
            <link rel="pre-1.0-signal" href="_pagefind" >
            <h1>Hello.</h1>
            """
        Given I have a "pagefind.yml" file with the content:
            """
            source: public
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see "pre-1.0 compatibility mode" in stderr
        Then I should see the file "public/_pagefind/pagefind.js"

    Scenario: LEGACY Settings can be pulled from JSON configuration files
        Given I have a "public/index.html" file with the body:
            """
            <link rel="pre-1.0-signal" href="_pagefind" >
            <h1>Hello.</h1>
            """
        Given I have a "pagefind.json" file with the content:
            """
            {
                "source": "public"
            }
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see "pre-1.0 compatibility mode" in stderr
        Then I should see the file "public/_pagefind/pagefind.js"

    Scenario: LEGACY Settings can be pulled from command-line flags
        Given I have a "public/index.html" file with the body:
            """
            <link rel="pre-1.0-signal" href="_pagefind" >
            <h1>Hello.</h1>
            """
        When I run my program with the flags:
            | --source public |
        Then I should see "Running Pagefind" in stdout
        Then I should see "pre-1.0 compatibility mode" in stderr
        Then I should see the file "public/_pagefind/pagefind.js"

    Scenario: LEGACY Settings can be pulled from environment variables
        Given I have a "public/index.html" file with the body:
            """
            <link rel="pre-1.0-signal" href="_pagefind" >
            <h1>Hello.</h1>
            """
        Given I have the environment variables:
            | PAGEFIND_SOURCE | public |
        When I run my program
        Then I should see "Running Pagefind" in stdout
        Then I should see "pre-1.0 compatibility mode" in stderr
        Then I should see the file "public/_pagefind/pagefind.js"

    Scenario: LEGACY Settings can be pulled from multiple sources
        Given I have a "public/index.html" file with the body:
            """
            <link rel="pre-1.0-signal" href="_pagefind" >
            <h1>Hello.</h1>
            """
        Given I have a "pagefind.json" file with the content:
            """
            {
                "source": "public"
            }
            """
        When I run my program with the flags:
            | --bundle-dir _out |
        Then I should see "Running Pagefind" in stdout
        Then I should see the file "public/_out/pagefind.js"
