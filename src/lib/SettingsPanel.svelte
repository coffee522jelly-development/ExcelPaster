<script lang="ts">
  import { Settings } from "lucide-svelte";

  export let showSettings: boolean;
  export let leftToken: string;
  export let rightToken: string;
  export let leftHeader: string;
  export let rightHeader: string;
  export let onTokenBlur: () => void;
</script>

<div class="flex justify-end mb-4">
  <button
    class="flex items-center gap-2 px-4 py-2 bg-gray-500 hover:bg-gray-600 text-white rounded text-sm font-semibold transition-colors"
    on:click={() => showSettings = !showSettings}
  >
    <Settings size={16} />
    {showSettings ? '設定を閉じる' : '設定を開く'}
  </button>
</div>

{#if showSettings}
  <div class="bg-white p-4 rounded-lg shadow mb-4 flex flex-col gap-4">
    <h3 class="m-0 border-b border-gray-100 pb-2 text-lg font-semibold">設定</h3>

    <div class="flex items-center gap-2">
      <label class="font-bold w-40" for="leftToken">左画像識別子</label>
      <input
        id="leftToken"
        type="text"
        class="border border-gray-300 rounded px-2 py-1 flex-1"
        bind:value={leftToken}
        on:blur={onTokenBlur}
      />
    </div>
    <div class="flex items-center gap-2">
      <label class="font-bold w-40" for="rightToken">右画像識別子</label>
      <input
        id="rightToken"
        type="text"
        class="border border-gray-300 rounded px-2 py-1 flex-1"
        bind:value={rightToken}
        on:blur={onTokenBlur}
      />
    </div>
    <div class="flex items-center gap-2">
      <label class="font-bold w-40" for="leftHeader">左画像エクセル文言</label>
      <input
        id="leftHeader"
        type="text"
        class="border border-gray-300 rounded px-2 py-1 flex-1"
        bind:value={leftHeader}
      />
    </div>
    <div class="flex items-center gap-2">
      <label class="font-bold w-40" for="rightHeader">右画像エクセル文言</label>
      <input
        id="rightHeader"
        type="text"
        class="border border-gray-300 rounded px-2 py-1 flex-1"
        bind:value={rightHeader}
      />
    </div>
  </div>
{/if}
