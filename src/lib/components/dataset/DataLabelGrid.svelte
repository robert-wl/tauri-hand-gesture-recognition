<script lang="ts">
  import TauriService from "../../../services/tauri-service";
  import DatasetLabelCard from "./DatasetLabelCard.svelte";

  export let name: string;
  const data = Array.from({ length: 40 }).map((i) => ({
    id: i,
    name: `Item ${i}`,
  }));
  console.log(data);
  const fetchData = async () => {
    return await TauriService.getDatasetLabel(name);
  };
</script>

<div class="w-full h-full grid grid-cols-6 gap-10">
  {#await fetchData()}
    <h1>Loading...</h1>
  {:then labels}
    {#each labels as item}
      <DatasetLabelCard datasetLabel={item} />
    {/each}
  {/await}
</div>
