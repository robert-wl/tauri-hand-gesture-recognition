<script lang="ts">
  const stateColor = {
    loading: "progress-warning",
    processed: "progress-success",
    unprocessed: "progress-base-100",
  };

  export let progress = 0;
</script>

<progress
  class={`progress w-full bg-base-500 h-1 ${progress >= 100 ? stateColor.processed : stateColor.loading}`}
  max="100"
  value={progress}></progress>
