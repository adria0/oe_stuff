pragma solidity >=0.4.21 <0.7.0;

contract Demo {
    uint public balance;

    // Initialize global variables
    constructor() public 
    {
        balance = 0;
    }

    // The payable keyword allows this function to accept Ether
    function contribute() public payable
    {
        // msg.value is the value of Ether sent in a transaction
        balance += msg.value;
    }
}
