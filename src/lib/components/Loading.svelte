<script>
  import SpinnerIcon from "./icons/SpinnerIcon.svelte";
  import { onMount } from "svelte";

  export let centered = true;

  let number = 0;

  onMount(() => {
    const interval = setInterval(() => {
      number += 1;
      number %= 5;
      console.log(number);
    }, 1000);

    return () => clearInterval(interval);
  });
</script>

<p class="flex flex-row text-lg gap-4 items-center justify-center {centered ? '' : 'pe-[20%]'}">
  <SpinnerIcon
    height="0.5rem"
    width="0.5rem" />
  Loading {#each Array.from({ length: number }) as _}
    .{" "}
  {/each}
</p>
