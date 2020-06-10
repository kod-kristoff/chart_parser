from collections import namedtuple
from itertools import islice

Rule = namedtuple("Rule", "lhs rhs")
Rule.__str__ = lambda self: "%s --> %s" % (self.lhs, " ".join(map(str, self.rhs)))

Edge = namedtuple("Edge", "start end lhs rhs dot result", defaults=[None])
Edge.__str__ = lambda e: (
    "[%s-%s: %s --> %s . %s]"
    % (e.start, e.end, e.lhs, " ".join(e.rhs[: e.dot]), " ".join(e.rhs[e.dot :]))
)
Edge.passive = lambda e: e.dot == len(e.rhs)


def earley1(grammar, input):
    # The 0th edgeset is always empty, because there are no edges ending in position 0
    chart = [set()]

    # Enumerate each word in the input, starting from k=1
    for k, word in enumerate(input, 1):
        print(f"word {k}: {word}")
        edgeset = set()

        # Scan: add one single passive edge for the input word
        agenda = [Edge(k - 1, k, word, (), 0)]

        while agenda:
            print("agenda = ", agenda)
            edge = agenda.pop()
            print("edge = ", edge)
            # Add the edge to the edgeset (if it's not already there)
            if edge not in edgeset:
                edgeset.add(edge)

                # If the edge is passive we can apply the inference rules
                if edge.passive():

                    # Predict: find grammar rules looking for edge.lhs
                    agenda.extend(
                        Edge(edge.start, k, lhs, rhs, 1)
                        for (lhs, rhs) in grammar
                        if edge.lhs == rhs[0]
                    )

                    # Complete: find active edges in old edgesets looking for edge.lhs
                    agenda.extend(
                        Edge(e.start, k, e.lhs, e.rhs, e.dot + 1)
                        for e in chart[edge.start]
                        if not e.passive()
                        if edge.lhs == e.rhs[e.dot]
                    )

        # Add the edgeset to the end of the chart
        chart.append(edgeset)

    # Filter all passive edges from the chart, and return them
    return [[e for e in edgeset if e.passive()] for edgeset in chart]


def success(chart, cat, start=0, end=-1):
    print("chartsize = ", chartsize(chart))
    return any(
        edge.start == start and edge.lhs == cat and edge.passive()
        for edge in chart[end]
    )


def chartsize(chart):
    return sum(map(len, chart))


def print_chart(chart, positions=None, cutoff=None):
    print("Chart size: %d edges" % chartsize(chart))
    for (k, edgeset) in enumerate(chart):
        if edgeset and (
            positions is None or k in positions or (k - len(chart)) in positions
        ):
            print("%d edges ending in position %s:" % (len(edgeset), k))
            for n, edge in enumerate(sorted(edgeset)):
                if cutoff and n >= cutoff:
                    print("    ...")
                    break
                print("   ", edge)


def example(n):
    prefix = "the lion sees a zebra".split()
    suffix = "under a tree with a telescope in the park".split()
    return prefix + (suffix * (n // 3 + 1))[: n * 3]


if __name__ == "__main__":
    r = Rule("S", ("NP", "VP"))
    print("str(r) = %s" % (r,))
    print("repr(r) = %r" % (r,))
    print("r.lhs = %s, r.rhs = %s" % (r.lhs, r.rhs))
    grammar = [
        Rule("S", ("NP", "VP")),
        Rule("VP", ("Verb",)),
        Rule("VP", ("Verb", "NP")),
        Rule("VP", ("VP", "PP")),
        Rule("NP", ("Det", "Noun")),
        Rule("NP", ("NP", "PP")),
        Rule("PP", ("Prep", "NP")),
        Rule("Verb", ("sees",)),
        Rule("Det", ("the",)),
        Rule("Det", ("a",)),
        Rule("Prep", ("under",)),
        Rule("Prep", ("with",)),
        Rule("Prep", ("in",)),
        Rule("Noun", ("zebra",)),
        Rule("Noun", ("lion",)),
        Rule("Noun", ("tree",)),
        Rule("Noun", ("park",)),
        Rule("Noun", ("telescope",)),
    ]

    for rule in grammar:
        print(rule)

    for n in range(5):
        print("example(%d) = %s" % (n, " ".join(example(n))))

    edge1 = Edge(0, 2, "S", ("NP", "VP"), 1)
    print(edge1)
    edge2 = Edge(0, 5, "S", ("NP", "VP"), 2)
    print(edge2)

    sent1 = example(0)
    sent2 = sent1[:6]
    print('Parsing "%s": %s' % (" ".join(sent1), success(earley1(grammar, sent1), "S")))
    # print('Parsing "%s": %s' % (" ".join(sent2), success(earley1(grammar, sent2), "S")))
