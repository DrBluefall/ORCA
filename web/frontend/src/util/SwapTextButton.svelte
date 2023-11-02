<script lang="ts">
    export let initial_text: string = "Text 1";
    export let swap_text: string = "Text Two";
    export let callback = () => {};
    let current: string = initial_text;
</script>

<button
    on:mouseenter={() => {
        current = swap_text;
    }}
    on:mouseleave={() => {
        current = initial_text;
    }}
    on:click|preventDefault={callback}
    {...$$restProps}>
    {current}
</button>

<style lang="scss">
    $backgroundColor: #000000;
    $color: #ffffff;

    * {
        box-sizing: border-box;
    }

    button {
        font-size: 1.15rem;
        border: none;
        background-color: transparent;
        font-family: inherit;
        padding: 0;
        cursor: pointer;

        @media screen and (-ms-high-contrast: active) {
            border: 2px solid currentcolor;
        }
    }

    button {
        // Display
        display: inline-flex;
        align-items: center;
        justify-content: center;

        // Visual
        background-color: $backgroundColor;
        color: $color;
        border-radius: 8px;
        box-shadow: 0 3px 5px rgba(0, 0, 0, 0.18);

        // Size
        padding: 0.25em 0.75em;
        min-width: 10ch;
        min-height: 44px;

        // Text
        text-align: center;
        line-height: 1.1;

        // Spacing
        margin-top: 5px;
        margin-bottom: 5px;

        transition: 220ms all ease-in-out;

        &:hover,
        &:active {
            background-color: $color;
            color: $backgroundColor;
        }
    }
</style>
