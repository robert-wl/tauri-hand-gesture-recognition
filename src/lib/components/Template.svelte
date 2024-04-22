<script lang="ts">
  import NProgress from "nprogress";
  import BackIcon from "./icons/BackIcon.svelte";
  import { navigate } from "svelte-routing";
  import CloseIcon from "./icons/CloseIcon.svelte";
  import { scale } from "svelte/transition";
  import { exit } from "@tauri-apps/api/process";

  export let title: string;
  export let description: string;

  NProgress.done();

  let currentUrl = new URL(window.location.href);

  const goBack = () => {
    currentUrl = new URL(window.location.href);
    const newUrl = currentUrl.pathname.split("/").reverse()?.[1] ?? "";
    navigate(`/${newUrl}`, { replace: true });
  };

  const exitApp = () => {
    exit(0);
  };
</script>

<div class="w-full h-full flex flex-col justify-start items-center gap-4 p-4 relative">
  {#if currentUrl.pathname !== "/"}
    <button
      in:scale|global={{ duration: 200, opacity: 0.5, start: 0.5 }}
      class="absolute top-5 left-5 bg-primary hover:bg-primary/90 transition-colors text-white p-2 rounded-full cursor-pointer shadow-md"
      on:click={goBack}>
      <BackIcon
        height="1.25rem"
        width="1.25rem" />
    </button>
  {/if}
  <button
    class="absolute top-5 right-5 bg-red-500 hover:bg-red-500/90 transition-colors text-white p-2 rounded-full cursor-pointer shadow-md"
    on:click={exitApp}>
    <CloseIcon
      height="1.25rem"
      width="1.25rem" />
  </button>
  <div class="flex flex-col py-5 gap-5 items-center">
    <h1 class="text-4xl font-bold text-center mt-16">{title}</h1>
    <p class="text-center text-lg max-w-[40rem]">{description}</p>
  </div>
  <slot />
</div>
