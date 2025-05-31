#include <iostream>

using namespace std;

#define MASK 0b00001111
#define PAT  0b00001000

bool bits_match(unsigned bin) {
    return (bin & MASK) == PAT;
}

int main() {
    for (unsigned i=0x07; i<0xfa; i++) {
        cout.width(2);
        cout.fill('0');
        cout << hex << i;
        cout << " : " << bits_match(i) << endl;
    }
}
