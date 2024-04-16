<script>
  import MenuCard from "../lib/components/MenuCard.svelte";
</script>


<div class="w-full h-full flex flex-col justify-start items-center gap-4 p-4">
  <div class="flex flex-col py-5 gap-5 items-center">
    <h1 class="text-4xl font-bold text-center mt-24">Hand Gesture Recognition</h1>
    <p class="text-center text-lg max-w-[40rem]">
      A Machine Learning Project to Recognize Hand Gestures, Preprocessed using MediaPipe
      Framework and Trained using Support Vector Machine Model
    </p>
  </div>
  <div class="w-full flex flex-wrap justify-center items-center gap-4 p-4">
    <a href="/preprocess">
      <MenuCard
        imageUrl="/home/data-preprocess.png"
        text="Preprocess Hand Images with MediaPipe Framework"
        title="Data Preprocessing"
      />
    </a>
    <MenuCard
      imageUrl="/home/model-training.png"
      text="Train a Support Vector Machine Model with to classify Hand Gestures"
      title="Model Training"
    />
    <MenuCard
      imageUrl="/home/model-testing.png"
      text="Test the Model on New Hand Gestures"
      title="Model Testing"
    />
  </div>
</div>