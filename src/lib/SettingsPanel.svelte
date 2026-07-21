<script lang="ts">
  import { Settings, X } from "lucide-svelte";
  import { Dialog } from "bits-ui";

  export let showSettings: boolean;
  export let leftToken: string;
  export let rightToken: string;
  export let extraToken: string;
  export let leftHeader: string;
  export let rightHeader: string;
  export let extraHeader: string;
  export let leftColor: string;
  export let rightColor: string;
  export let extraColor: string;
  export let authorName: string;
  export let registeredSystems: string;
  export let selectedSystem: string;
  export let onTokenBlur: () => void;
</script>

<Dialog.Root bind:open={showSettings}>
  <Dialog.Trigger
    class="p-1 text-slate-500 hover:text-slate-800 dark:text-slate-400 dark:hover:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-full transition-colors focus:outline-none"
  >
    <Settings size={18} />
  </Dialog.Trigger>
  <Dialog.Portal>
    <Dialog.Overlay class="fixed inset-0 bg-black/50 z-50 transition-opacity" />
    <Dialog.Content
      class="fixed right-0 top-0 h-full w-[300px] max-w-full bg-white dark:bg-slate-900 p-4 shadow-xl z-50 flex flex-col gap-4 transform transition-transform duration-300 ease-in-out border-l border-slate-200 dark:border-slate-800"
    >
      <div class="flex items-center justify-between border-b border-slate-200 dark:border-slate-800 pb-2">
        <Dialog.Title class="text-base font-bold text-slate-800 dark:text-slate-100">設定</Dialog.Title>
        <Dialog.Close
          class="p-1 rounded hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-500 hover:text-slate-800 dark:hover:text-slate-200 transition-colors"
        >
          <X size={16} />
        </Dialog.Close>
      </div>

      <div class="flex flex-col gap-3">
        <div class="flex flex-col gap-1">
          <label class="font-semibold text-xs text-slate-700 dark:text-slate-300" for="authorName">氏名</label>
          <input
            id="authorName"
            type="text"
            class="border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-950 text-slate-900 dark:text-slate-100 rounded px-2 py-1 text-xs focus:ring-1 focus:ring-blue-500 focus:outline-none"
            bind:value={authorName}
          />
        </div>
        <div class="flex flex-col gap-1">
          <label class="font-semibold text-xs text-slate-700 dark:text-slate-300" for="registeredSystems">登録システム名 (カンマ区切り)</label>
          <input
            id="registeredSystems"
            type="text"
            class="border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-950 text-slate-900 dark:text-slate-100 rounded px-2 py-1 text-xs focus:ring-1 focus:ring-blue-500 focus:outline-none"
            bind:value={registeredSystems}
            placeholder="システムA,システムB"
          />
        </div>
        <div class="flex flex-col gap-1">
          <label class="font-semibold text-xs text-slate-700 dark:text-slate-300" for="selectedSystemSettings">デフォルトシステム</label>
          <select
            id="selectedSystemSettings"
            class="border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-950 text-slate-900 dark:text-slate-100 rounded px-2 py-1 text-xs focus:ring-1 focus:ring-blue-500 focus:outline-none"
            bind:value={selectedSystem}
          >
            {#each registeredSystems.split(',').map(s => s.trim()).filter(s => s) as sys}
              <option value={sys}>{sys}</option>
            {/each}
          </select>
        </div>
        <div class="w-full h-px bg-slate-200 dark:bg-slate-700 my-1"></div>
        <div class="flex flex-col gap-1">
          <label class="font-semibold text-xs text-slate-700 dark:text-slate-300" for="leftToken">左画像識別子</label>
          <input
            id="leftToken"
            type="text"
            class="border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-950 text-slate-900 dark:text-slate-100 rounded px-2 py-1 text-xs focus:ring-1 focus:ring-blue-500 focus:outline-none"
            bind:value={leftToken}
            on:blur={onTokenBlur}
          />
        </div>
        <div class="flex flex-col gap-1">
          <label class="font-semibold text-xs text-slate-700 dark:text-slate-300" for="rightToken">右画像識別子</label>
          <input
            id="rightToken"
            type="text"
            class="border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-950 text-slate-900 dark:text-slate-100 rounded px-2 py-1 text-xs focus:ring-1 focus:ring-blue-500 focus:outline-none"
            bind:value={rightToken}
            on:blur={onTokenBlur}
          />
        </div>
        <div class="flex flex-col gap-1">
          <label class="font-semibold text-xs text-slate-700 dark:text-slate-300" for="extraToken">補足画像識別子</label>
          <input
            id="extraToken"
            type="text"
            class="border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-950 text-slate-900 dark:text-slate-100 rounded px-2 py-1 text-xs focus:ring-1 focus:ring-blue-500 focus:outline-none"
            bind:value={extraToken}
            on:blur={onTokenBlur}
          />
        </div>
        <div class="flex flex-col gap-1">
          <label class="font-semibold text-xs text-slate-700 dark:text-slate-300" for="leftHeader">左画像エクセル文言</label>
          <input
            id="leftHeader"
            type="text"
            class="border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-950 text-slate-900 dark:text-slate-100 rounded px-2 py-1 text-xs focus:ring-1 focus:ring-blue-500 focus:outline-none"
            bind:value={leftHeader}
          />
        </div>
        <div class="flex flex-col gap-1">
          <label class="font-semibold text-xs text-slate-700 dark:text-slate-300" for="rightHeader">右画像エクセル文言</label>
          <input
            id="rightHeader"
            type="text"
            class="border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-950 text-slate-900 dark:text-slate-100 rounded px-2 py-1 text-xs focus:ring-1 focus:ring-blue-500 focus:outline-none"
            bind:value={rightHeader}
          />
        </div>
        <div class="flex flex-col gap-1">
          <label class="font-semibold text-xs text-slate-700 dark:text-slate-300" for="extraHeader">補足画像エクセル文言</label>
          <input
            id="extraHeader"
            type="text"
            class="border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-950 text-slate-900 dark:text-slate-100 rounded px-2 py-1 text-xs focus:ring-1 focus:ring-blue-500 focus:outline-none"
            bind:value={extraHeader}
          />
        </div>
        <div class="flex flex-col gap-1">
          <label class="font-semibold text-xs text-slate-700 dark:text-slate-300" for="leftColor">左側セル背景色</label>
          <input
            id="leftColor"
            type="color"
            class="border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-950 rounded h-8 w-full cursor-pointer focus:ring-1 focus:ring-blue-500 focus:outline-none"
            bind:value={leftColor}
          />
        </div>
        <div class="flex flex-col gap-1">
          <label class="font-semibold text-xs text-slate-700 dark:text-slate-300" for="rightColor">右側セル背景色</label>
          <input
            id="rightColor"
            type="color"
            class="border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-950 rounded h-8 w-full cursor-pointer focus:ring-1 focus:ring-blue-500 focus:outline-none"
            bind:value={rightColor}
          />
        </div>
        <div class="flex flex-col gap-1">
          <label class="font-semibold text-xs text-slate-700 dark:text-slate-300" for="extraColor">補足セル背景色</label>
          <input
            id="extraColor"
            type="color"
            class="border border-slate-300 dark:border-slate-700 bg-white dark:bg-slate-950 rounded h-8 w-full cursor-pointer focus:ring-1 focus:ring-blue-500 focus:outline-none"
            bind:value={extraColor}
          />
        </div>
      </div>

    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
