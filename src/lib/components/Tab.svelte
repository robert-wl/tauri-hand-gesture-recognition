<script lang="ts">
  export let tabContent: string[] = [];
  export let activeTab: number = 0;
  export let setActiveTab: (index: number) => void;
</script>

<div class="flex flex-col w-full items-center justify-center transition-colors">
  <ul class="flex flex-wrap w-full items-center justify-center -mb-px text-sm font-medium text-center text-gray-500 border-b border-gray-200">
    {#each tabContent as tab}
      <li class="flex-1">
        <button
          on:click={() => setActiveTab(tabContent.indexOf(tab))}
          class="{tabContent[activeTab] === tab
            ? 'text-blue-600 border-b-2 border-blue-600'
            : 'hover:text-gray-600 hover:border-gray-300 border-b-2 border-transparent'} inline-flex w-4/5 items-center justify-center p-4 rounded-t-lg group">
          {tab}
        </button>
      </li>
    {/each}
  </ul>
</div>
