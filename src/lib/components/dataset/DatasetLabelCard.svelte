<script lang="ts">
  import type { DatasetLabel } from "../../../../bindings";

  export let datasetLabel: DatasetLabel;
</script>

<button>
  <div class="card min-h-40 bg-base-100 hover:bg-gray-50 shadow-xl cursor-pointer duration-300 transition-all hover:-translate-y-2">
    <div class="card-body items-center">
      <h2 class="card-title text-xl text-center pb-2">{datasetLabel.name}</h2>
      <div class="w-48 bg-gray-400 rounded-md">
        <img
          alt="Dataset Thumbnail"
          class="object-cover rounded-md transition-opacity"
          src={`data:image/jpeg;base64,${datasetLabel.thumbnail}`} />
      </div>
      <hr class="my-2 w-full border-gray-200" />
      <p class="text-center">{datasetLabel.data_amount} Images</p>
    </div>
  </div>
</button>
