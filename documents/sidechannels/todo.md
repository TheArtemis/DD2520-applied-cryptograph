### Side-Channel Countermeasure Task

Modify your AES code to protect it against **cache-based access-driven side-channel attacks**.

### Possible Countermeasures

Examples of possible countermeasures include:

- **Allocation of the AES table in non-cacheable memory**
- **Flushing the cache**
- **Selective eviction of cache lines** that can be affected by secret information
- **Priming the cache** with data that will be used
- **Use of special AES assembler instructions**
- **Randomization of tables**
- **Spreading each AES table entry over multiple cache lines**

Select a countermeasure that fits your **programming language and environment** (for example, it is not possible to use AES instruction extensions if you are not using C/C++ or a similar language).  
In some cases, even if your language does not directly support certain features (e.g., cache priming), the same behavior can be implemented by standard operations (e.g., reading all elements of the S-Box and assigning the result to a variable).

### Written Report

Write a small document (e.g., **2 pages**) that:

- **Explains why your chosen countermeasure is effective**, and  
- **Documents the assumptions** you have made about your compiler, programming language, and execution environment.

Examples of such assumptions:

- I assumed that the stack is allocated in a different region from the S-Box.
- I assumed the compiler allocates the S-Box contiguously.
- I assumed cache lines are 64 bits.
- Etc.

Upload:

- **Code** as a `.txt` file.
- **Document** as a `.pdf` file.

You do not need to present this.

### Peer Review

Write a **0.5-page peer review** of the countermeasure of another student (you will be assigned a peer review later).  
When done, upload your peer review document as a `.pdf`.
