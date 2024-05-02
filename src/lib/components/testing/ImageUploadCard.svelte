<script lang="ts">
  import { scale } from "svelte/transition";
  import UploadIcon from "../icons/UploadIcon.svelte";
  import CrossIcon from "../icons/CrossIcon.svelte";

  export let file: Nullable<File>;

  const handleFileInput = (e: Event) => {
    const target = e.target as HTMLInputElement;
    file = target.files![0];
  };

  const removeImage = (e: MouseEvent) => {
    e.preventDefault();
    file = null;
  };

  $: imageUrl = file ? URL.createObjectURL(file) : null;
</script>

<div
  class="card w-[35rem] min-h-40 bg-base-100 shadow-xl border-primary border-t-2"
  in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}>
  <div class="card-title flex flex-col justify-center items-center w-full py-2 gap-0">
    <h2 class="text-xl font-bold">Image Upload</h2>
    <hr class="my-2 w-full border-gray-200" />
  </div>
  <div class="card-body items-center">
    <div class="flex items-center justify-center w-full">
      <label
        class="flex flex-col items-center justify-center w-full h-64 border-2 {imageUrl ??
          'border-dashed border-accent cursor-pointer'} group rounded-lg bg-gray-50 hover:bg-gray-100"
        for="dropzone-file">
        {#if imageUrl}
          <img
            alt="preview"
            class="w-full h-full rounded-md object-cover"
            src={imageUrl} />
          <button
            on:click={removeImage}
            class="opacity-0 hover:bg-gray-300 transition-all duration-200 bg-white rounded-full text-red-500 group-hover:opacity-100 flex items-center justify-center absolute top-1/2 right-1/2 transform translate-x-1/2">
            <CrossIcon
              width="3rem"
              height="3rem" />
          </button>
        {:else}
          <div class="flex flex-col items-center justify-center pt-5 pb-6">
            <UploadIcon />
            <p class="mb-2 text-sm text-gray-500">
              <span class="font-semibold">Click to upload</span>
            </p>
          </div>
          <input
            class="hidden"
            id="dropzone-file"
            on:change={handleFileInput}
            type="file" />
        {/if}
      </label>
    </div>
  </div>
</div>
