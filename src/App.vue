<script lang="ts" setup>
import {
    Avatar,
    Button,
    DataTable,
    FloatLabel,
    InputNumber,
    Panel,
    Select
} from "primevue";
import Chart from "primevue/chart";
import {onMounted, ref} from "vue";
import Column from 'primevue/column';
import {UserInfo} from "./bo/user.ts";
import {LogLevel} from "./bo/common.ts";

// optional
interface LogMessage {
    code: string;
    name: string;
    category: string;
    quantity: number;
}

let userSelectOptions: UserInfo[] = [];
onMounted(() => {
    chartData.value = setChartData();
    chartOptions.value = setChartOptions();
    logMessages.value = [{
        code: "code1",
        name: "name1",
        category: "category1",
        quantity: 1
    }, {
        code: "code1",
        name: "name1",
        category: "category1",
        quantity: 1
    }, {
        code: "code1",
        name: "name1",
        category: "category1",
        quantity: 1
    }];
});
const chartData = ref();
const chartOptions = ref();
const logMessages = ref<LogMessage[]>();
const setChartData = () => {
    const documentStyle = getComputedStyle(document.documentElement);
    return {
        labels: ['January', 'February', 'March', 'April', 'May', 'June', 'July'],
        datasets: [
            {
                label: 'First Dataset',
                data: [65, 59, 80, 81, 56, 55, 40],
                fill: false,
                borderColor: documentStyle.getPropertyValue('--p-cyan-500'),
                tension: 0.4
            },
            {
                label: 'Second Dataset',
                data: [28, 48, 40, 19, 86, 27, 90],
                fill: false,
                borderColor: documentStyle.getPropertyValue('--p-gray-500'),
                tension: 0.4
            }
        ]
    };
};
const setChartOptions = () => {
    const documentStyle = getComputedStyle(document.documentElement);
    const textColor = documentStyle.getPropertyValue('--p-text-color');
    const textColorSecondary = documentStyle.getPropertyValue('--p-text-muted-color');
    const surfaceBorder = documentStyle.getPropertyValue('--p-content-border-color');
    return {
        maintainAspectRatio: false,
        aspectRatio: 0.6,
        plugins: {
            legend: {
                labels: {
                    color: textColor
                }
            }
        },
        scales: {
            x: {
                ticks: {
                    color: textColorSecondary
                },
                grid: {
                    color: surfaceBorder
                }
            },
            y: {
                ticks: {
                    color: textColorSecondary
                },
                grid: {
                    color: surfaceBorder
                }
            }
        }
    };
}
</script>

<template>
    <main
        class="m-1 min-w-100 flex flex-col">
        <Panel class="m-3 min-w-100 max-w-130" header="User information"
               toggleable>
            <div class="flex flex-row justify-items-center items-center">
                <Avatar class="mr-3" icon="pi pi-user"
                        size="xlarge"/>
                <div
                    class="flex flex-col">
                    <div class="uppercase font-bold">Remote server</div>
                    <div>127.0.0.1:10080</div>
                </div>
            </div>

        </Panel>
        <Panel class="m-3 min-w-100 max-w-130" header="User configuration"
               toggleable>
            <FloatLabel class="w-full" variant="on">
                <Select id="select_user" :options="userSelectOptions"
                        class="w-full">
                    <template #dropdownicon>
                        <i class="pi pi-user"/>
                    </template>
                </Select>
                <label for="select_user">Select a user</label>
            </FloatLabel>
        </Panel>

        <Panel class="m-3 min-w-100 max-w-130"
               header="Connection configuration" toggleable>
            <div>
                <FloatLabel class="w-full" variant="on">
                    <InputNumber id="agent_server_port"
                                 :max="65535" :min="1" :useGrouping="false"
                                 class="w-full mt-3 mb-3"></InputNumber>
                    <label for="agent_server_port">Agent server port</label>
                </FloatLabel>
                <FloatLabel class="w-full" variant="on">
                    <InputNumber id="worker_thread_number"
                                 :max="65535" :min="1" :useGrouping="false"
                                 class="w-full mt-3 mb-3"></InputNumber>
                    <label for="worker_thread_number">Worker thread
                        number</label>
                </FloatLabel>
                <FloatLabel class="w-full" variant="on">
                    <Select id="max_log_level"
                            :options="[LogLevel.ERROR, LogLevel.INFO, LogLevel.WARNING, LogLevel.DEBUG]"
                            class="w-full mt-3 mb-3">
                        <template #dropdownicon>
                            <i class="pi pi-sliders-h"/>
                        </template>
                    </Select>
                    <label for="max_log_level">Max log level</label>
                </FloatLabel>

            </div>
        </Panel>
        <Panel class="m-3 min-w-100 max-w-130"
               header="Connection pool configuration" toggleable>
            <div>
                <FloatLabel class="w-full mt-3 mb-3" variant="on">
                    <InputNumber id="max_pool_size"
                                 :max="65535" :min="1" :useGrouping="false"
                                 class="w-full"></InputNumber>
                    <label for="max_pool_size">Max pool size</label>
                </FloatLabel>
                <FloatLabel class="w-full mt-3 mb-3" variant="on">
                    <InputNumber id="fill_interval"
                                 :max="60" :min="0" :useGrouping="false"
                                 class="w-full"></InputNumber>
                    <label for="fill_interval">Fill interval</label>
                </FloatLabel>
                <FloatLabel class="w-full mt-3 mb-3" variant="on">
                    <InputNumber id="check_interval"
                                 :max="60" :min="0" :useGrouping="false"
                                 class="w-full"></InputNumber>
                    <label for="check_interval">Check interval</label>
                </FloatLabel>

            </div>
        </Panel>


        <Panel class="m-3 min-w-100 max-w-130 "
               header="Download/Upload speed"
               toggleable>
            <Chart :data="chartData" :options="chartOptions"
                   class="w-full h-[20rem]" type="line"/>
        </Panel>
        <Panel class="m-3 min-w-100 max-w-130" header="Log console" toggleable>
            <DataTable :value="logMessages">
                <Column field="code" header="Code"></Column>
                <Column field="name" header="Name"></Column>
                <Column field="category" header="Category"></Column>
                <Column field="quantity" header="Quantity"></Column>
            </DataTable>

        </Panel>

        <Panel class="m-3 min-w-100 max-w-130 ">
            <div class="flex justify-center items-center content-center gap-4">
                <Button class="w-29 uppercase" label="Start"></Button>
                <Button class="w-29 uppercase" label="Stop"></Button>
                <Button class="w-36 uppercase"
                        label="Import user"></Button>
            </div>
        </Panel>

    </main>
</template>

<style>
:root {
    margin: 0;
    padding: 0;
    font-family: arial, helvetica, sans-serif, "Microsoft YaHei", '宋体';
}

</style>