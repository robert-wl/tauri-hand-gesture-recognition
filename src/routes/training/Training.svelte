<script lang="ts">
  import DatasetService from "../../services/dataset-service";

  export let name: string = "";

  let isLoading = false;

  const getModelDataset = async () => {
    return await DatasetService.getModelDataset();
  };

  const trainModel = async () => {
    isLoading = true;
    await DatasetService.trainModel();
    isLoading = false;
  };
</script>

<div class="w-full h-full flex flex-col justify-start items-center gap-4 p-4">
  <div class="flex flex-col py-5 gap-5 items-center">
    <h1 class="text-4xl font-bold text-center mt-16">Preprocess</h1>
    <p class="text-center text-lg max-w-[40rem]">Detect the landmarks in the images and collect the data for each image.</p>
  </div>
  <div class="w-full flex flex-row justify-start items-start gap-10 p-4">
    <div>
      <div class="card w-64 min-h-80 bg-base-100 shadow-xl sticky top-0 border-primary border-t-2">
        <div class="card-title pb-0 py-4 flex flex-col items-center justify-between">
          <img
            alt="dataset"
            class="w-32 h-32 rounded-full"
            src="/training/model.png" />
          <h2 class="text-xl font-bold">{name}</h2>
          <hr class="my-2 w-full border-gray-200" />
        </div>
        <div class="flex flex-col w-full justify-center items-center gap-4">
          <label class="form-control w-4/5 max-w-xs">
            <div class="label">
              <span class="label-text">Select dataset</span>
            </div>
            <select class="select bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100">
              <option
                disabled
                selected>Pick one</option>
              {#await getModelDataset() then dataset}
                {#each dataset as d}
                  <option>{d.name}</option>
                {/each}
              {/await}
            </select>
          </label>
          <label class="form-control w-4/5 max-w-xs">
            <div class="label">
              <span class="label-text">Model Name</span>
            </div>
            <input
              class="input bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100"
              placeholder="Jumbotron 3000"
              type="text" />
          </label>
          <label class="form-control w-4/5 max-w-xs">
            <div class="label">
              <span class="label-text">Select kernel</span>
            </div>
            <select class="select bg-gray-50 hover:border-gray-300 border-gray-300 focus:outline-none focus:scale-100">
              <option selected>RBF</option>
              <option>Linear</option>
              <option>Polynomial</option>
              <option>Sigmoid</option>
            </select>
          </label>
        </div>
        <div class="card-body pt-2 flex flex-col h-full items-start justify-start">
          <div class="flex flex-col items-start flex-1">
            <!--            <div class="text-center">Total Types: {dataset.labels.length}</div>-->
            <!--            <div class="text-center">Total Images: {dataLength}</div>-->
            <!--            <div class="text-center">Processed Images: 0/{dataLength}</div>-->
          </div>
          {#if isLoading}
            <button
              class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-2 w-full"
              disabled>
              Loading...
            </button>
          {:else}
            <button class="btn btn-primary btn-sm text-white min-h-0 h-fit font-bold py-2.5 mt-2 w-full">
              <!--              on:click={preprocessDataset}>-->
              Train Model
            </button>
          {/if}
        </div>
      </div>
    </div>
    <div class="w-full h-full grid 2xl:grid-cols-6 grid-cols-4 gap-10">
      <!--{#each dataset.labels as label}-->
      <!--  <DatasetLabelCard-->
      <!--    datasetName={name}-->
      <!--    datasetLabel={label} />-->
      <!--{/each}-->
    </div>
  </div>
</div>
