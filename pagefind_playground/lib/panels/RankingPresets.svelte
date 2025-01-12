<script lang="ts">
    import { pagefindRankingDefaults } from "../defaults";

    import { onMount } from "svelte";

    let {
        settings,
        updateSettings,
    }: {
        settings: Record<string, number>;
        updateSettings: (to: Record<string, number>) => void;
    } = $props();

    let settingPresets: Record<string, Record<string, number>> = $state({});
    let allSettingPresets: Record<string, Record<string, number>> = $derived({
        ...settingPresets,
        "Pagefind Default": pagefindRankingDefaults,
    });
    let selected: string = $state("Pagefind Default");

    onMount(() => {
        try {
            const storedSettings = localStorage.getItem(
                "pagefindRankingSettings",
            );
            if (storedSettings) {
                settingPresets = JSON.parse(storedSettings);
            }
        } catch (e) {
            console.error("Unable to load stored ranking presets", e);
        }
    });

    const handleLoad = (e: Event) => {
        if (e.target instanceof HTMLFormElement) {
            const formData = new FormData(e.target);
            const newSettings =
                allSettingPresets[formData.get("loadPreset")?.toString() || ""];
            if (newSettings) {
                updateSettings(newSettings);
            }
        }
    };

    const handleCreate = (e: Event) => {
        if (e.target instanceof HTMLFormElement) {
            const formData = new FormData(e.target);
            const name = formData.get("createPreset")?.toString() || "Untitled";
            settingPresets[name] = { ...settings };
            e.target.reset();
            selected = name;

            localStorage.setItem(
                "pagefindRankingSettings",
                JSON.stringify(settingPresets),
            );
        }
    };

    const handleDelete = (e: Event) => {
        settingPresets = {};
        selected = "Pagefind Default";
        updateSettings(allSettingPresets["Pagefind Default"]);

        localStorage.setItem(
            "pagefindRankingSettings",
            JSON.stringify(settingPresets),
        );
    };
</script>

<form action="javascript:void(0);" onsubmit={handleLoad}>
    <div class="row">
        <label for="loadPreset">Load Preset</label>
        <select id="loadPreset" name="loadPreset" bind:value={selected}>
            {#each Object.keys(allSettingPresets) as preset}
                <option value={preset}>{preset}</option>
            {/each}
        </select>
        <button type="submit">Load</button>
    </div>
</form>
<form action="javascript:void(0);" onsubmit={handleCreate}>
    <div class="row">
        <label for="createPreset">Create Preset</label>
        <input type="text" id="createPreset" name="createPreset" required />
        <button type="submit">Save</button>
    </div>
</form>
<div class="r-row">
    <button type="button" onclick={handleDelete}>Delete all presets</button>
</div>

<style>
    form {
        margin-bottom: 8px;
    }

    .row {
        max-width: 500px;
        display: grid;
        grid-template-columns: 150px 1fr auto;
        align-items: center;
        gap: 8px;
    }

    .r-row {
        max-width: 500px;
        display: flex;
        justify-content: flex-end;
    }

    @container (max-width: 420px) {
        .row {
            grid-template-columns: 1fr auto;
        }
        label {
            grid-column: span 2;
        }
    }

    input,
    select {
        box-sizing: border-box;
        color: var(--fg);
        border: solid 1px var(--fg);
        background-color: var(--bg);
        padding: 0 6px;
        width: 100%;
        font-size: 16px;
        height: 36px;
    }

    button {
        color: var(--fg);
        border: solid 1px var(--fg);
        background-color: var(--bg);
        padding: 0 12px;
        height: 36px;
        cursor: pointer;
    }
    button:hover {
        background-color: var(--sub-bg);
    }

    label {
        font-size: var(--fz);
    }
</style>
