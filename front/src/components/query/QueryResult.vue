<template>
    <div class="col-12">
        <div class="result-list-item">
            <div class="result-list-detail">
                <div class="result-name">{{ result.name }}</div>
            </div>
        </div>
    </div>
</template>
  
<script lang="ts" setup>

interface IResult {
    name: string
}

interface Props {
    result: IResult,
}

const props = defineProps<Props>();

</script>
<style lang="scss" scoped>
.result-name {
    font-size: 1.5rem;
    font-weight: 700;
}

.result-description {
    margin: 0 0 1rem 0;
}

.result-category-icon {
    vertical-align: middle;
    margin-right: .5rem;
}

.result-category {
    font-weight: 600;
    vertical-align: middle;
}

::v-deep(.result-list-item) {
    display: flex;
    align-items: center;
    padding: 1rem;
    width: 100%;

    img {
        width: 50px;
        box-shadow: 0 3px 6px rgba(0, 0, 0, 0.16), 0 3px 6px rgba(0, 0, 0, 0.23);
        margin-right: 2rem;
    }

    .result-list-detail {
        flex: 1 1 0;
    }

    .p-rating {
        margin: 0 0 .5rem 0;
    }

    .result-price {
        font-size: 1.5rem;
        font-weight: 600;
        margin-bottom: .5rem;
        align-self: flex-end;
    }

    .result-list-action {
        display: flex;
        flex-direction: column;
    }

    .p-button {
        margin-bottom: .5rem;
    }
}

::v-deep(.result-grid-item) {
    margin: .5rem;
    border: 1px solid var(--surface-border);

    .result-grid-item-top,
    .result-grid-item-bottom {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    img {
        box-shadow: 0 3px 6px rgba(0, 0, 0, 0.16), 0 3px 6px rgba(0, 0, 0, 0.23);
        margin: 2rem 0;
    }

    .result-grid-item-content {
        text-align: center;
    }

    .result-price {
        font-size: 1.5rem;
        font-weight: 600;
    }
}

@media screen and (max-width: 576px) {
    .result-list-item {
        flex-direction: column;
        align-items: center;

        img {
            margin: 2rem 0;
        }

        .result-list-detail {
            text-align: center;
        }

        .result-price {
            align-self: center;
        }

        .result-list-action {
            display: flex;
            flex-direction: column;
        }

        .result-list-action {
            margin-top: 2rem;
            flex-direction: row;
            justify-content: space-between;
            align-items: center;
            width: 100%;
        }
    }
}
</style>