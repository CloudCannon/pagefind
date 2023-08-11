export const style = new CSSStyleSheet();
style.replaceSync(/* css */`
    [data-pfmod-hidden] {
        display: none !important;
    }

    [data-pfmod-suppressed] {
        opacity: 0 !important;
        pointer-events: none !important;
    }

    [data-pfmod-sr-hidden] {
        -webkit-clip: rect(0 0 0 0) !important;
        clip: rect(0 0 0 0) !important;
        -webkit-clip-path: inset(100%) !important;
        clip-path: inset(100%) !important;
        height: 1px !important;
        overflow: hidden !important;
        overflow: clip !important;
        position: absolute !important;
        white-space: nowrap !important;
        width: 1px !important;
    }

    [data-pfmod-loading] {
        color: var(--pagefind-ui-text);
        background-color: var(--pagefind-ui-text);
        border-radius: var(--pagefind-ui-border-radius);
        opacity: 0.1;
        pointer-events: none;
    }
`);

