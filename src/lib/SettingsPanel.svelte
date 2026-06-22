<script lang="ts">
  import { Settings, X } from "lucide-svelte";
  import { Dialog } from "bits-ui";

  export let showSettings: boolean;
  export let leftToken: string;
  export let rightToken: string;
  export let leftHeader: string;
  export let rightHeader: string;
  export let onTokenBlur: () => void;
</script>

<Dialog.Root bind:open={showSettings}>
  <Dialog.Trigger
    class="p-2 text-gray-500 hover:text-gray-800 hover:bg-gray-100 rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-gray-300"
  >
    <Settings size={24} />
  </Dialog.Trigger>
  <Dialog.Portal>
    <Dialog.Overlay class="fixed inset-0 bg-black/50 z-50 transition-opacity" />
    <Dialog.Content
      class="fixed right-0 top-0 h-full w-[400px] max-w-full bg-white p-6 shadow-xl z-50 flex flex-col gap-6 transform transition-transform duration-300 ease-in-out"
    >
      <div class="flex items-center justify-between border-b pb-4">
        <Dialog.Title class="text-xl font-bold text-gray-800">設定</Dialog.Title>
        <Dialog.Close
          class="p-1 rounded hover:bg-gray-100 text-gray-500 hover:text-gray-800 transition-colors"
        >
          <X size={20} />
        </Dialog.Close>
      </div>

      <div class="flex flex-col gap-4">
        <div class="flex flex-col gap-2">
          <label class="font-bold text-sm text-gray-700" for="leftToken">左画像識別子</label>
          <input
            id="leftToken"
            type="text"
            class="border border-gray-300 rounded px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:outline-none"
            bind:value={leftToken}
            on:blur={onTokenBlur}
          />
        </div>
        <div class="flex flex-col gap-2">
          <label class="font-bold text-sm text-gray-700" for="rightToken">右画像識別子</label>
          <input
            id="rightToken"
            type="text"
            class="border border-gray-300 rounded px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:outline-none"
            bind:value={rightToken}
            on:blur={onTokenBlur}
          />
        </div>
        <div class="flex flex-col gap-2">
          <label class="font-bold text-sm text-gray-700" for="leftHeader">左画像エクセル文言</label>
          <input
            id="leftHeader"
            type="text"
            class="border border-gray-300 rounded px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:outline-none"
            bind:value={leftHeader}
          />
        </div>
        <div class="flex flex-col gap-2">
          <label class="font-bold text-sm text-gray-700" for="rightHeader">右画像エクセル文言</label>
          <input
            id="rightHeader"
            type="text"
            class="border border-gray-300 rounded px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:outline-none"
            bind:value={rightHeader}
          />
        </div>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
