<script lang="ts">
    let {
        settings,
        updateSetting,
    }: {
        settings: Record<string, number>;
        updateSetting: (name: string, to: number) => void;
    } = $props();

    const handleUpdate = (e: Event) => {
        if (e.target instanceof HTMLInputElement) {
            updateSetting(e.target.id, parseFloat(e.target.value));
        }
    };
</script>

<form action="javascript:void(0);">
    <div class="row">
        <label for="termFrequency">Term Frequency</label>
        <code>{settings.termFrequency}</code>
        <input
            type="range"
            min="0.0"
            max="1.0"
            step="0.01"
            id="termFrequency"
            value={settings.termFrequency}
            oninput={handleUpdate}
        />
    </div>
    <div class="row">
        <label for="termSimilarity">Term Similarity</label>
        <code>{settings.termSimilarity}</code>
        <input
            type="range"
            min="0.0"
            max="10.0"
            step="0.01"
            id="termSimilarity"
            value={settings.termSimilarity}
            oninput={handleUpdate}
        />
    </div>
    <div class="row">
        <label for="pageLength">Page Length</label>
        <code>{settings.pageLength}</code>
        <input
            type="range"
            min="0.0"
            max="1.0"
            step="0.01"
            id="pageLength"
            value={settings.pageLength}
            oninput={handleUpdate}
        />
    </div>
    <div class="row">
        <label for="termSaturation">Term Saturation</label>
        <code>{settings.termSaturation}</code>
        <input
            type="range"
            min="0.0"
            max="2.0"
            step="0.01"
            id="termSaturation"
            value={settings.termSaturation}
            oninput={handleUpdate}
        />
    </div>
</form>

<style>
    .row {
        display: grid;
        grid-template-columns: 150px 50px 300px;
        align-items: center;
        gap: 8px;
    }

    label {
        font-size: var(--fz);
    }
</style>
