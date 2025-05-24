<script setup lang="ts">
import { Stat } from '@/model/chat';
import { Ref, ref, watch } from 'vue';


const props = defineProps<{
	stat: Stat;
}>()

let currentVal:Ref<number> = ref(0)
function change(){
	if(currentVal.value === props.stat.value) return;

	if(props.stat.value - currentVal.value > 1 ){
		currentVal.value += 1;
	}else{
		currentVal.value = props.stat.value;
	}

	setTimeout(change, 20)
}

watch(
	() => props.stat,
	change,
	{ immediate: true }
)

</script>

<template>
	<div class="gauge" v-if="stat">
		<span class="title">{{ stat.name }}</span>
		<span class="value">{{ (Math.round(currentVal * 100) / 100).toString() }}</span>
	</div>
	<div v-else></div>
</template>


<style lang="css" scoped>
.gauge {
	display: flex;
	flex-direction: column;
	align-items: center;
}

.gauge>.value {
	font-size: 2em;
	font-weight: bold;
}

.gauge>.title {
	text-align: center;
}
</style>