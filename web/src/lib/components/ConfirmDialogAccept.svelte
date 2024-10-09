<script lang="ts">

  import ConfirmDialog from "./ConfirmDialog.svelte"; 
  export let hidden : boolean = false;
  export let iconSize : string = "25px";
  export let acceptEnabled : boolean = true;

</script>

<ConfirmDialog hidden={hidden} iconSize={iconSize} acceptEnabled={acceptEnabled} showAccept={true}>
  <slot></slot>
</ConfirmDialog>
