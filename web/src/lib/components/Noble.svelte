<script lang="ts">
  import queen from "$lib/assets/queen2.svg";
</script>

<div class="noble-container">
  <img src={queen} alt="Queen" draggable="false"/>
  <div class="details"> 
    <slot></slot>
  </div>
</div>

<style>
  .noble-container {
    width: 100%;
    height: 20%;
    position : relative;
    border-radius: 20px;
    flex-direction: row;
    display: flex;
  }

  .details {
    display: flex;
    justify-content: left;
    flex-direction: column;
    gap: 5%;
    width: 50%;
    height: 100%;
    position: absolute;
    top: 0;
    left: 50%;
  }
  img {
    width: 40%;
    height: 100%;
  }
</style>
