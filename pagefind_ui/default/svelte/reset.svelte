<script></script>

<style>
    /***
    The new CSS reset - version 1.7.2 (last updated 23.6.2022)
    GitHub page: https://github.com/elad2412/the-new-css-reset
***/

    /*
    Remove all the styles of the "User-Agent-Stylesheet", except for the 'display' property
    - The "symbol *" part is to solve Firefox SVG sprite bug
 */
    :global(.pagefind-ui--reset
            *:where(:not(html, iframe, canvas, img, svg, video):not(svg
                    *, symbol *))) {
        all: unset;
        display: revert;
        outline: revert;
    }

    /* Preferred box-sizing value */
    :global(.pagefind-ui--reset *, .pagefind-ui--reset
            *::before, .pagefind-ui--reset *::after) {
        box-sizing: border-box;
    }

    /* Reapply the pointer cursor for anchor tags */
    :global(.pagefind-ui--reset a, .pagefind-ui--reset button) {
        cursor: revert;
    }

    /* Remove list styles (bullets/numbers) */
    :global(.pagefind-ui--reset ol, .pagefind-ui--reset ul, .pagefind-ui--reset
            menu) {
        list-style: none;
    }

    /* For images to not be able to exceed their container */
    :global(.pagefind-ui--reset img) {
        max-width: 100%;
    }

    /* removes spacing between cells in tables */
    :global(.pagefind-ui--reset table) {
        border-collapse: collapse;
    }

    /* Safari - solving issue when using user-select:none on the <body> text input doesn't working */
    :global(.pagefind-ui--reset input, .pagefind-ui--reset textarea) {
        -webkit-user-select: auto;
    }

    /* revert the 'white-space' property for textarea elements on Safari */
    :global(.pagefind-ui--reset textarea) {
        white-space: revert;
    }

    /* minimum style to allow to style meter element */
    :global(.pagefind-ui--reset meter) {
        -webkit-appearance: revert;
        appearance: revert;
    }

    /* reset default text opacity of input placeholder */
    :global(.pagefind-ui--reset ::placeholder) {
        color: unset;
    }

    /* fix the feature of 'hidden' attribute.
   display:revert; revert to element instead of attribute */
    :global(.pagefind-ui--reset :where([hidden])) {
        display: none;
    }

    /* revert for bug in Chromium browsers
   - fix for the content editable attribute will work properly.
   - webkit-user-select: auto; added for Safari in case of using user-select:none on wrapper element*/
    :global(.pagefind-ui--reset
            :where([contenteditable]:not([contenteditable="false"]))) {
        -moz-user-modify: read-write;
        -webkit-user-modify: read-write;
        overflow-wrap: break-word;
        -webkit-line-break: after-white-space;
        -webkit-user-select: auto;
    }

    /* apply back the draggable feature - exist only in Chromium and Safari */
    :global(.pagefind-ui--reset :where([draggable="true"])) {
        -webkit-user-drag: element;
    }

    /* Pagefind specific — use the default mark styling */
    :global(.pagefind-ui--reset mark) {
        all: revert;
    }
</style>
