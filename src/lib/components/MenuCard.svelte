<script lang="ts">
  export let imageUrl: string;
  export let title: string;
  export let text: string;


</script>


<div class="card w-80 min-h-[32rem] bg-white hover:bg-gray-50 shadow-xl cursor-pointer duration-300 transition-all hover:-translate-y-2">
  <figure class="p-8">
    <img alt={title} src={imageUrl} />
  </figure>
  <div class="card-body items-center">
    <h2 class="card-title text-center">{title}</h2>
    <p class="text-center">{text}</p>
  </div>
</div>