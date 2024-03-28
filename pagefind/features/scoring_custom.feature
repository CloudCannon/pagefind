Feature: Result Scoring
    Background:
        Given I have the environment variables:
            | PAGEFIND_SITE | public |
        Given I have a "public/index.html" file with the body:
            """
            <ul>
                <li data-result>
            </ul>
            """
        # Create dummy pages to allow BM25 calculations to be effective
        Given I have a "public/latin-1/index.html" file with the body:
            """
            <p>Maecenas sed diam eget risus varius blandit sit amet non common</p>
            """
        Given I have a "public/latin-2/index.html" file with the body:
            """
            <p>Cras justo odio, common ac facilisis in, egestas eget quam.</p>
            """
        Given I have a "public/latin-3/index.html" file with the body:
            """
            <p>Donec sed odio dui.</p>
            """
        Given I have a "public/latin-4/index.html" file with the body:
            """
            <p>Vivamus sagittis lacus vel augue laoreet rutrum faucibus dolor auctor.</p>
            """
        Given I have a "public/latin-5/index.html" file with the body:
            """
            <p>Integer posuere erat a ante venenatis dapibus posuere velit aliquet..</p>
            """

    Scenario: Term similarity ranking can be configured
        Given I have a "public/similar-term/index.html" file with the body:
            """
            <p>This post talks about abcdef once</p>
            """
        Given I have a "public/dissimilar-term/index.html" file with the body:
            """
            <p>This post talks about abcdefghijklmnopqrstuv — twice! abcdefghijklmnopqrstuv</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        # The abcdefghijklmnopqrstuv hits should be pretty useless
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search(`abcdef`);

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/similar-term/, /dissimilar-term/"
        # The abcdefghijklmnopqrstuv hits are just as important, so win due to two of them
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");
                await pagefind.options({
                    ranking: {
                        termSimilarity: 0.0
                    }
                });

                let search = await pagefind.search(`abcdef`);

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/dissimilar-term/, /similar-term/"

    Scenario: Page length ranking can be configured
        Given I have a "public/longer/index.html" file with the body:
            """
            <p>This post is quite long, and talks about terracotta at length.</p>
            <p>Fusce dapibus, tellus ac cursus commodo, tortor mauris condimentum nibh, ut fermentum terracotta justo sit amet risus. Donec sed odio dui. Aenean eu leo quam. Pellentesque ornare sem lacinia quam venenatis vestibulum. Nulla vitae elit libero, a pharetra augue. Aenean lacinia bibendum nulla sed consectetur. Donec id elit non mi porta gravida at eget metus. Maecenas faucibus mollis interdum.</p>
            <p>Integer terracotta erat a ante venenatis dapibus posuere velit aliquet. Vivamus sagittis lacus vel augue laoreet rutrum faucibus terracotta auctor. Nullam quis risus eget urna mollis ornare vel eu leo. Aenean lacinia bibendum nulla sed consectetur.</p>
            <p>Praesent commodo cursus magna, vel scelerisque terracotta consectetur et. Fusce dapibus, tellus ac cursus commodo, tortor mauris condimentum nibh, ut fermentum massa justo sit amet risus. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur blandit tempus porttitor.</p>
            """
        Given I have a "public/shorter/index.html" file with the body:
            """
            <p>This is a shorter terracotta page.</p>
            <p>Sed posuere consectetur est at lobortis.</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        # Should prefer documents shorter than the average document
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");
                await pagefind.options({
                    ranking: {
                        pageLength: 1.0
                    }
                });

                let search = await pagefind.search(`terracotta`);

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/shorter/, /longer/"
        # Should care about term frequency more than document length
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");
                await pagefind.options({
                    ranking: {
                        pageLength: 0.0
                    }
                });

                let search = await pagefind.search(`terracotta`);

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/longer/, /shorter/"

    Scenario: Term frequency vs raw count can be configured
        Given I have a "public/longer/index.html" file with the body:
            """
            <p>This post is quite long, and talks about terracotta at length.</p>
            <p>Fusce dapibus, tellus ac cursus commodo, tortor mauris condimentum nibh, ut fermentum terracotta justo sit amet risus. Donec sed odio dui. Aenean eu leo quam. Pellentesque ornare sem lacinia quam venenatis vestibulum. Nulla vitae elit libero, a pharetra augue. Aenean lacinia bibendum nulla sed consectetur. Donec id elit non mi porta gravida at eget metus. Maecenas faucibus mollis interdum.</p>
            <p>Integer erat a ante venenatis dapibus posuere velit aliquet. Vivamus sagittis lacus vel augue laoreet rutrum faucibus auctor. Nullam quis risus eget urna mollis ornare vel eu leo. Aenean lacinia bibendum nulla sed consectetur.</p>
            <p>Praesent commodo cursus magna, vel scelerisque consectetur et. Fusce dapibus, tellus ac cursus commodo, tortor mauris condimentum nibh, ut fermentum massa justo sit amet risus. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur blandit tempus porttitor.</p>
            """
        Given I have a "public/shorter/index.html" file with the body:
            """
            <p>This is a shorter terracotta page.</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        # Default: should score based on term frequency
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");

                let search = await pagefind.search(`terracotta`);

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/shorter/, /longer/"
        # Flipped: Should pick the page with higher result count, regardless of length
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");
                await pagefind.options({
                    ranking: {
                        termFrequency: 0.0
                    }
                });

                let search = await pagefind.search(`terracotta`);

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/longer/, /shorter/"

    Scenario: Term saturation can be configured
        Given I have a "public/lots/index.html" file with the body:
            """
            <h3>post</h3>
            <p>common and common and common and unrelated</p>
            """
        Given I have a "public/slightly-less-than-lots/index.html" file with the body:
            """
            <h1>post</h1>
            <p>common and common and unrelated and unrelated</p>
            """
        When I run my program
        Then I should see "Running Pagefind" in stdout
        When I serve the "public" directory
        When I load "/"
        # More sensitive to term frequency, should pick the more frequent document for "common"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");
                await pagefind.options({
                    ranking: {
                        termSaturation: 2.0
                    }
                });

                let search = await pagefind.search(`common post`);

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/slightly-less-than-lots/, /lots/"
        # Less sensitive to term frequency of "common", should pick the better document for "post"
        When I evaluate:
            """
            async function() {
                let pagefind = await import("/pagefind/pagefind.js");
                await pagefind.options({
                    ranking: {
                        termSaturation: 0.1
                    }
                });

                let search = await pagefind.search(`common post`);

                let data = await Promise.all(search.results.map(result => result.data()));
                document.querySelector('[data-result]').innerText = data.map(d => d.url).join(', ');
            }
            """
        Then There should be no logs
        Then The selector "[data-result]" should contain "/lots/, /slightly-less-than-lots/"
