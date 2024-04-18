<script lang="ts">
  import TauriService from "../../../services/tauri-service";
  import DatasetLabelCard from "./DatasetLabelCard.svelte";
  import DataPreviewModal from "./DataPreviewModal.svelte";
  import { getContext } from "svelte";
  import type { Dataset } from "../../../../bindings";

  let dataset = getContext<Dataset>("dataset");
  let dialog: HTMLDialogElement;

  let labels = getContext("labels");
  let chosenLabel = "";
  const openModal = (label: string) => {
    chosenLabel = label;
    dialog.showModal();
  };
</script>

<div class="w-full h-full grid 2xl:grid-cols-6 grid-cols-4 gap-10">
  {#each $dataset.labels as l}
    <DatasetLabelCard
      on:click={() => openModal(l.name)}
      databaseName={""}
      datasetLabel={l} />
  {/each}

  <DataPreviewModal
    bind:dialog
    datasetLabel={chosenLabel}
    datasetName={""} />
</div>
