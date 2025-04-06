<script lang="ts" setup>
import {Button, Fieldset, InputText, Panel} from "primevue";
</script>

<template>
    <main>
        <h1>Ppaass Agent</h1>
        <Fieldset legend="SERVER CONFIGURATION">
            <InputText placeholder="Port number"></InputText>
            <InputText placeholder="Thread number"></InputText>
        </Fieldset>
        <Fieldset legend="CONNECTION POOL CONFIGURATION">
            <InputText placeholder="Connection number"></InputText>
        </Fieldset>
        <Panel>
            <Button label="START"></Button>
        </Panel>
    </main>
</template>

<style>
:root {
    margin: 0;
    padding: 0;
}
</style>