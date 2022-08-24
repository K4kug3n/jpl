<div align="center">
  <div>
    <a href="https://github.com/K4kug3n/jpl/actions?query=workflow%3Alinux-build">
      <img src="https://github.com/K4kug3n/jpl/workflows/linux-build/badge.svg" alt="github-ci" />
    </a>
  </div>
  <div>
    <a href="https://github.com/K4kug3n/jpl/blob/main/LICENSE">
      <img src="https://img.shields.io/github/license/K4kug3n/jpl?style=plastic" alt="license" />
    </a>
  </div>

</div>

# Just a Programming Language

JPL is a toy language to practice programming language creation, made in Rust as learning project

## Current Grammar
Hardly inspired by Compilers & Interpreters course from University of Geneva  

PGRM &rarr; LIST_INSTR  

LIST_INSTR &rarr; INSTR LIST_INSTR  
LIST_INSTR &rarr; eps  
INSTR &rarr; id = E;  

E &rarr; T D  
D &rarr; + E  
D &rarr; - E  
D &rarr; eps  
T &rarr; F G  
G &rarr; * E  
G &rarr; / E  
G &rarr; eps  
F &rarr; ( E )  
F &rarr; id  
F &rarr; nb  