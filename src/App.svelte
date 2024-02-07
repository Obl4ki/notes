<script lang="ts">
  import { onMount } from "svelte";
  import {
    colorScheme,
    SvelteUIProvider,
    TextInput,
    Container,
    Stack,
    Accordion,
    Text,
    Button,
  } from "@svelteuidev/core";
  import { invoke } from "@tauri-apps/api";

  colorScheme.update(() => "dark");

  let noteTitle = "";
  let noteContent = "";

  let save_to_file = async () => {
    await invoke("save_to_file", {
      filename: noteTitle,
      content: noteTitle + "\n" + noteContent,
    });
    noteTitle = "";
    noteContent = "";
    await get_notes();
  };

  let notes: Note[] = [];

  let get_notes = async () => {
    await invoke("get_notes").then((_notes) => (notes = _notes as Note[]));
  };

  onMount(() => {
    get_notes();
  });

  $: console.log(notes);

  type Note = {
    id: number;
    title: string;
    content: string;
  };
</script>

<SvelteUIProvider withGlobalStyles themeObserver="dark">
  <Container>
    <form on:submit|preventDefault={save_to_file}>
      <TextInput bind:value={noteTitle} placeholder="..." label="Title" />
      <TextInput bind:value={noteContent} placeholder="..." label="Content" />
      <Button type="submit">Save</Button>
    </form>
    <Stack>
      <Accordion>
        {#if notes.length !== 0}
          {#each notes as note}
            <Accordion.Item value={note.title}>
              <Text slot="control" size="lg" align="center">{note.title}</Text>

              <Text size="md" align="justify">{note.content}</Text>
            </Accordion.Item>
          {/each}
        {:else}
          <Text align="center">Your notes are empty!</Text>
        {/if}
      </Accordion>
    </Stack>
  </Container>
</SvelteUIProvider>
