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
Inspired by Compilers & Interpreters course from University of Geneva  

PGRM &rarr; LIST_INSTR  

LIST_INSTR &rarr; INSTR LIST_INSTR  
LIST_INSTR &rarr; ε  
INSTR &rarr; let *id* = E;  
INSTR &rarr; *id* = E;  

E &rarr; T J  

J &rarr; != E  
J &rarr; == E  
J &rarr; >= E  
J &rarr; <= E 
J &rarr; > E  
J &rarr; < E  
J &rarr; ε  

T &rarr; H G  

G &rarr; * E  
G &rarr; / E  
G &rarr; && E  
G &rarr; ε  

H &rarr; F D  

D &rarr; + E  
D &rarr; - E  
D &rarr; || E  
D &rarr; ε  

F &rarr; ( E )  
F &rarr; *id*  
F &rarr; *nb*  
F &rarr; *bool*  