<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const greetMsg = ref('Do I change?');
const connectionString = ref('');

async function connect() {
  try {
    greetMsg.value = await invoke('connect', { connectionString: connectionString.value });
  } catch (error: any) {
    greetMsg.value = error;
  }
}
</script>

<template>
  <main class="min-h-screen w-full bg-slate-800 text-white">
    <div class="container mx-auto">
      <h1 class="text-lg">Document Migrator</h1>
      <p class="text-sm">
        Your friendly neighborhood data migrator that let's you live your best Postgres life.
      </p>
    </div>

    <form @submit.prevent="connect" class="container mx-auto mt-4">
      <label for="connectionString" class="block text-sm font-medium text-gray-300"
        >Connection String:</label
      >
      <input
        id="connectionString"
        v-model="connectionString"
        placeholder="mongodb://localhost:27017..."
        class="mt-1 block w-full px-3 py-2 bg-gray-700 text-white placeholder-gray-400 border border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
      />
      <button
        type="submit"
        class="mt-3 px-4 py-2 bg-indigo-600 text-white font-semibold rounded-md shadow hover:bg-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
      >
        Connect
      </button>
      <p>{{ greetMsg }}</p>
    </form>
  </main>
</template>
