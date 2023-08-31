Feature: Filtering
    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |
        Given I have a "public/index.html" file with the body:
            """
            <p data-results>Nothing</p>
            """
        Given I have a "public/cheeka/index.html" file with the body:
            """
            <span data-pagefind-filter="color">Black</span>
            <span data-pagefind-filter="color">White</span>
            <h1 data-pagefind-filter="mood:Lazy">Cat</h1>
            """
        Given I have a "public/theodore/index.html" file with the body:
            """
            <span data-pagefind-filter="mood">Zen</span>
            <span data-pagefind-filter="color">Orange</span>
            <h1 data-pagefind-filter="color:White">Cat</h1>
            """
        Given I have a "public/ali/index.html" file with the body:
            """
            <span data-pagefind-filter="mood">Angry</span>
            <h1 data-pagefind-filter="color:Tabby">Ali Cat</h1>
            """
        Given I have a "public/smudge/index.html" file with the body:
            """
            <span data-pagefind-filter="mood">Boisterous</span>
            <span data-pagefind-filter="mood">Mischievous</span>
            <span data-pagefind-filter="color">Black</span>
            <span data-pagefind-filter="color">White</span>
            <h1>Cat</h1>
            """
        Given I have a "public/grey/index.html" file with the body:
            """
            <span data-pagefind-filter="mood">Nervous</span>
            <span data-pagefind-filter="mood">Pining</span>
            <span data-pagefind-filter="color">Grey</span>
            <h1>Cat</h1>
            """
        Given I have a "public/treacle/index.html" file with the body:
            """
            <span data-pagefind-filter="mood">Lazy</span>
            <span data-pagefind-filter="color">Black</span>
            <span data-pagefind-filter="color">White</span>
            <span data-pagefind-filter="color">Orange</span>
            <h1>Cat</h1>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        When I evaluate:
            """
            async function() {
                window.pagefind = await import("/pagefind/pagefind.js");

                window.test = async function(pagefind_incantation) {
                    let search = await pagefind_incantation;
                    let data = await Promise.all(search.results.map(result => result.data()));

                    document.querySelector('[data-results]').innerText = data.map(d => d.url).sort().join(', ');
                }
            }
            """

    Scenario: Filtering to any of a single filter
        When I evaluate:
            """
            async function() {
                await test(pagefind.search("Cat", {
                    filters: {
                        color: {
                            any: ["Black", "Orange"]
                        }
                    }
                }));
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "/cheeka/, /smudge/, /theodore/, /treacle/"

    Scenario: Filtering to any of a set of values and another filter
        When I evaluate:
            """
            async function() {
                await test(pagefind.search("Cat", {
                    filters: {
                        color: {
                            any: ["Black", "Orange"]
                        },
                        mood: "Lazy"
                    }
                }));
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "/cheeka/, /treacle/"
        When I evaluate:
            """
            async function() {
                await test(pagefind.search("Cat", {
                    filters: {
                        color: {
                            any: ["Black", "Orange"]
                        },
                        mood: "Zen"
                    }
                }));
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "/theodore/"

    Scenario: Filtering to any of a set of values or another filter
        When I evaluate:
            """
            async function() {
                await test(pagefind.search("Cat", {
                    filters: {
                        any: {
                            color: {
                                any: ["Black", "Orange"]
                            },
                            mood: "Angry"
                        }
                    }
                }));
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "/ali/, /cheeka/, /smudge/, /theodore/, /treacle/"

    Scenario: Filtering to a complex nested filter
        When I evaluate:
            """
            async function() {
                await test(pagefind.search("Cat", {
                    filters: {
                        any: [{
                            color: {
                                any: [{
                                    any: ["Tabby"]
                                }, {
                                    all: ["Black", "White", "Orange"]
                                }]
                            }
                        }, {
                            mood: {
                                all: ["Nervous", "Pining"]
                            }
                        }]
                    }
                }));
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "/ali/, /grey/, /treacle/"

    Scenario: Filtering with an exclusion
        When I evaluate:
            """
            async function() {
                await test(pagefind.search("Cat", {
                    filters: {
                        color: {
                            any: ["Black", "Orange"]
                        },
                        mood: {
                            not: "Lazy"
                        }
                    }
                }));
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "/smudge/, /theodore/"

    Scenario: Filtering with nested exclusions
        When I evaluate:
            """
            async function() {
                await test(pagefind.search("Cat", {
                    filters: {
                        all: [
                            {
                                all: [
                                    {
                                        color: {
                                            any: ["Orange", "White"]
                                        },
                                        mood: {
                                            any: ["Lazy", "Zen"]
                                        }
                                    },
                                    {
                                        not: {
                                            color: "Black"
                                        }
                                    }
                                ]
                            },
                            {
                                mood: {
                                    none: ["Lazy", "Nervous"]
                                }
                            }
                        ],
                    }
                }));
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "/theodore/"

    Scenario: Filtering with top-level exclusion
        When I evaluate:
            """
            async function() {
                await test(pagefind.search("Cat", {
                    filters: {
                        none: [
                            {
                                color: {
                                    any: ["Orange", "White"]
                                }
                            },
                            {
                                mood: "Angry"
                            }
                        ]
                    }
                }));
            }
            """
        Then There should be no logs
        Then The selector "[data-results]" should contain "/grey/"
