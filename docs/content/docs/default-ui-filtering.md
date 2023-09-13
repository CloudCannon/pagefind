---
title: "Filtering results with the Pagefind Default UI"
nav_title: "Filtering with the Default UI"
nav_section: Filtering
weight: 11
---

Pagefind's Default UI supports filters and will show them in a sidebar if they are present in your index.

The Default UI will also show a count beside each filter, representing the number of results available within that filter, taking the current search term and toggled filters into account.  
By default, filters with no remaining pages will still be shown. This can be disabled by setting the [`showEmptyFilters`](/docs/ui/#show-empty-filters) option to `false`.

Currently, the Default UI treats all filters as "AND" filters, meaning pages will only be shown if they match all toggled filters.
