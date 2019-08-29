pragma solidity ^0.4.0;

// This is a contract
contract SimpleStorage {
    uint storedData;

    // This is a setter
    function set(uint x) public {
        storedData = x;
    }

    // This is a getter
    function get() public view returns (uint) {
        return storedData;
    }
}
