# @version 0.3.7

"""
@title Avesome dummy contract
@author Lido
@license MIT
"""

from vyper.interfaces import ERC20


interface SomeInterface:
    def some_method() -> address: nonpayable


event SomeEvent:
    some_arg: indexed(address)
    yet_another_arg: uint256


some_var: public(address)
yet_another_var: public(uint256)


@external
def __init__():
    """
    @notice Initialize contract.
    """
    # Something important
    self.some_var = empty(address)


@external
@nonreentrant("lock")
def some_method(
    some_arg: uint256,
) -> bool:
    """
    @notice Also very important method.
    @dev This method is very important.
    @param some_arg Some argument.
    """
    assert self.some_var == empty(address), "we really need this"

    log SomeEvent(
        self.some_var,
        some_arg,
    )

    return True


@internal
@view
def _important_internal_view() -> uint256:
    return 0


@external
@view
def important_external_view() -> uint256:
    """
    @notice Yup, this is important too.
    """
    # NOTE: always return 154
    return 154
