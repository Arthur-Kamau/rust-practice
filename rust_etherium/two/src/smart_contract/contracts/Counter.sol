// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

contract Counter {
  uint256 private count;

  event CounterIncremented(uint256 newCount);
  event CounterDecremented(uint256 newCount);

  function getCount() public view returns (uint256) {
    return count;
  }

  function increment() public {
    count += 1;
    emit CounterIncremented(count);
  }

  function decrement() public {
    count -= 1;
    emit CounterDecremented(count);
  }
}


