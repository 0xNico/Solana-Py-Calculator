# calculator
# Built with Seahorse v0.1.6

from seahorse.prelude import *

declare_id('6Et5YsqS2LiMefFmf7iwQ2L2vmxmrzHHJeeuH12Nboou')

# Declare the calculator class with owners public key and i64 display
class Calculator(Account):
    owner: Pubkey
    display: i64

class Operation(Enum):
    ADD = 0
    SUB = 1
    MUL = 2
    DIV = 3

@instruction
def init_calculator(owner: Signer, calculator: Empty[Calculator]):
    # Initialize the calculator
    calculator = calculator.init(
        payer = owner,
        seeds = ['Calculator', owner]
    )
    calculator.owner = owner.key()

@instruction
def reset_calculator(owner: Signer, calculator: Calculator):
    # Print who is reseting calculator.
    print(owner.key(), 'has reset', calculator.key())
    # Veriify the owner of the calculator
    assert owner.key() == calculator.owner, 'This is not your calculator ser'
    # Set the display to zero
    calculator.display = 0


@instruction
def do_operation(owner: Signer, calculator: Calculator, op: Operation, num: i64):
    # Verify the Owner
    assert owner.key() == calculator.owner, 'This isnt your calculator ser'

    # Operation Logic
    if op == Operation.ADD:
        calculator.display += num
    elif op == Operation.SUB:
        calculator.display -= num
    elif op == Operation.MUL:
        calculator.display *= num
    elif op == Operation.DIV:
        calculator.display /= num