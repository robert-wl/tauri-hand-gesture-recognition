<script lang="ts">
  import type { Label } from "../../../../bindings";

  export let dialog: HTMLDialogElement;
  export let datasetLabel: Optional<Label>;

  let chosenData: string = "";

  $: if (datasetLabel) {
    chosenData = datasetLabel.data[0];
  }
</script>

<dialog
  bind:this={dialog}
  class="modal"
  id="data_modal">
  <div class="modal-box flex flex-row">
    <div class="flex flex-row w-full">
      <form method="dialog">
        <!-- if there is a button in form, it will close the modal -->
        <button class="btn">Close</button>
      </form>
    </div>
    <div class="flex flex-col min-w-20">
      {#if datasetLabel}
        {#each datasetLabel.data as dataset, i}
          <div class="flex flex-row">
            {dataset}
          </div>
        {/each}
      {/if}
    </div>
  </div>
</dialog>
