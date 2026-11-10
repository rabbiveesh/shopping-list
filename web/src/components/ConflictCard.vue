<script setup lang="ts">
import { computed } from 'vue'

import type { Conflict, ItemOp } from '@/core'
import { editLabel, fieldLabel } from '@/conflicts'

const props = defineProps<{ conflict: Conflict; itemName: string }>()
const emit = defineEmits<{ choose: [op: ItemOp] }>()

// A field conflict's options are all `set` edits of the same field.
const options = computed(() =>
  props.conflict.kind === 'field'
    ? props.conflict.options.flatMap((c) => (c.edit.op === 'set' ? [c.edit.value] : []))
    : [],
)
</script>

<template>
  <v-card variant="tonal" color="warning" class="mb-4 pa-4">
    <template v-if="conflict.kind === 'field'">
      <p class="mb-3">
        <strong>{{ itemName }}</strong
        >: {{ fieldLabel[options[0]!.field].toLowerCase() }} was changed on two phones at once.
        Which is right?
      </p>
      <v-btn
        v-for="edit in options"
        :key="editLabel(edit)"
        block
        variant="outlined"
        class="mb-2"
        @click="emit('choose', { op: 'set', value: edit })"
      >
        {{ editLabel(edit) }}
      </v-btn>
    </template>
    <template v-else>
      <p class="mb-3">
        <strong>{{ itemName }}</strong> was deleted on one phone while it was changed on another.
      </p>
      <v-btn
        block
        variant="outlined"
        class="mb-2"
        @click="emit('choose', { op: 'deleted', value: false })"
      >
        Keep it
      </v-btn>
      <v-btn block variant="outlined" @click="emit('choose', { op: 'deleted', value: true })">
        Delete it
      </v-btn>
    </template>
  </v-card>
</template>
