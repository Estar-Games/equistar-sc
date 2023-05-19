// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            4
// Async Callback (empty):               1
// Total number of exported functions:   6

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    equistar_game
    (
        updatePriceInEstar
        updatePriceInEgld
        buyTickets
        getTotalTicketsPerAddress
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
