<script lang="ts">
  import type { Label } from "../../../../bindings";
  import DatasetService from "../../../services/dataset-service";
  import SearchIcon from "../icons/SearchIcon.svelte";
  import RefreshIcon from "../icons/RefreshIcon.svelte";

  export let dialog: HTMLDialogElement;
  export let datasetName: string;
  export let datasetLabel: Optional<Label>;

  let rawImage: Optional<string>;
  let processedImage: Nullable<string>;
  let chosenData: string = "";
  let selectedTab: number = 0;

  const getImages = async () => {
    if (!datasetLabel) {
      return;
    }
    rawImage = await DatasetService.getRawImage(datasetName, datasetLabel.name, chosenData);
    processedImage = await DatasetService.getProcessedImage(datasetName, datasetLabel.name, chosenData);
  };

  $: if (datasetLabel) {
    chosenData = datasetLabel.data[0];
    getImages();
  }
</script>

<dialog
  bind:this={dialog}
  class="modal"
  id="data_modal">
  <div class="modal-box flex flex-col min-w-[50%] h-[70%] max-h-[70%] p-0">
    <div class="flex flex-col h-[10%]">
      <div>aa</div>
      <hr class="my-2 w-full border-gray-200" />
    </div>
    <div class="flex flex-row gap-2 max-h-[90%]">
      <div class="w-full flex flex-col group">
        <div
          class="tabs tabs-bordered flex"
          role="tablist">
          <input
            aria-label="Raw Image"
            checked={selectedTab === 0}
            class="tab flex flex-auto min-h-12"
            name="my_tabs_2"
            on:click={() => (selectedTab = 0)}
            role="tab"
            type="radio" />
          <input
            aria-label="Processed Image"
            checked={selectedTab === 1}
            class="tab flex flex-auto min-h-12"
            disabled={!processedImage}
            name="my_tabs_2"
            on:click={() => (selectedTab = 1)}
            role="tab"
            type="radio" />
        </div>
        <div>
          <!--          <img-->
          <!--            alt=""-->
          <!--            class="w-full group-[has:]"-->
          <!--            src="data:image/jpeg;base64,{selectedTab === 0 ? rawImage : processedImage}" />-->
        </div>
      </div>
      <div class="flex flex-col relative min-w-64 max-h-full flex-grow overflow-y-auto overflow-x-hidden gap-2">
        <div class="flex flex-row w-full sticky top-0 bg-white">
          <label class="input input-bordered flex w-full items-center gap-2">
            <input
              class="grow"
              placeholder="Search"
              type="text" />
            <svg
              class="w-4 h-4 opacity-70"
              fill="currentColor"
              viewBox="0 0 16 16"
              xmlns="http://www.w3.org/2000/svg"
              ><path
                clip-rule="evenodd"
                d="M9.965 11.026a5 5 0 1 1 1.06-1.06l2.755 2.754a.75.75 0 1 1-1.06 1.06l-2.755-2.754ZM10.5 7a3.5 3.5 0 1 1-7 0 3.5 3.5 0 0 1 7 0Z"
                fill-rule="evenodd" /></svg>
          </label>
        </div>
        {#if datasetLabel}
          {#each datasetLabel.data as dataset}
            <div class="flex flex-row p-2">
              {dataset}
            </div>
          {/each}
        {/if}
      </div>
    </div>
  </div>
</dialog>
